// disk.rs
#![allow(dead_code)]


use core::ptr;
use core::mem::size_of;
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use alloc::string::{String, ToString};

use crate::libuefi::bootservices::{BootServices, LocateSearchType};
use crate::libuefi::protocol::block_io::BlockIOProtocol;
use crate::libuefi::protocol::device_path::{ACPIDevicePath, DevicePathProtocol};
use crate::libuefi::protocol::loaded_image::LoadedImageProtocol;


static mut EFI_BLOCK_DEVICES: Vec<EFIBlockDevice> = Vec::new();

const BLOCK_SIZE: usize = 512;


struct EFIBlockDevice {
    name: String,
    handle: *const usize,
    is_slice: bool,
}

impl EFIBlockDevice {
    pub fn new(name: &str, handle: *const usize, is_slice: bool) -> Self {
        let name = name.to_string();
        Self {
            name,
            handle,
            is_slice,
        }
    }
}




/// Reads data from the disk as Box<T>
pub fn read_bytes_into_box<T>(dev: &str, lba: u64, count: usize) -> Box<T> {
    assert!(count <= size_of::<T>());

    let mut t: Box<T> = unsafe { Box::new(core::mem::zeroed()) };

    let buffer = read_bytes(dev, lba, count).unwrap();
    unsafe { core::ptr::copy(buffer.as_ptr(), (t.as_mut() as *mut T).cast(), count); }

    t
}






/// Reads data from the disk into T
pub fn read_bytes_into<T>(dev: &str, lba: u64, count: usize, buffer: &mut T) {
    assert!(count <= core::mem::size_of_val(buffer));
    unsafe { read_bytes_raw(dev, lba, count, (buffer as *mut T).cast()).unwrap(); }
}







pub fn read_bytes(dev: &str, lba: u64, count: usize) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = vec![0; count];

    match unsafe { read_bytes_raw(dev, lba, count, buffer.as_mut_ptr()) } {
        Ok(_) => Ok(buffer),
        Err(error) => Err(error)
    }
}








/// Reads bytes from the disk into a buffer. 'buffer' is a raw ptr, and is unsafe as the boundaries cannot be checked.
/// 
/// count: Number of bytes to read
/// lba: Logical block address, which logical block to start reading from
/// device: Deivce to read from. e.g disk0s1
/// buffer: Buffer to fill with bytes
pub unsafe fn read_bytes_raw(device: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    // If the count is smaller than the block size, UEFI error out. To mitigate this we create our own buffer can hold at least one block, read the block into it, then copy the requested amount of bytes into the callers buffer
    if count < BLOCK_SIZE {
        let mut tmp: Vec<u8> = vec![0; BLOCK_SIZE];

        match _uefi_read_bytes_raw(device, lba, BLOCK_SIZE, tmp.as_mut_ptr()) {
            Ok(_) => {
                ptr::copy(tmp.as_ptr(), buffer.cast(), count);
                Ok(())
            }

            Err(error) => Err(error)
        }
    }

    else {
        _uefi_read_bytes_raw(device, lba, count, buffer)
    }
}






unsafe fn _uefi_read_bytes_raw(dev: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::dangling_mut();
    BootServices::handle_protocol(find_device(dev).expect("Device not found.").handle, &(BlockIOProtocol::guid()), block_io_protocol.cast());
    let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };

    if count < BLOCK_SIZE {
        let mut tmp: Vec<u8> = vec![0; BLOCK_SIZE];
        let status = block_io_protocol.read_blocks(lba, BLOCK_SIZE, tmp.as_mut_ptr());
        if status == 0 {
            unsafe { ptr::copy(tmp.as_ptr(), buffer, count) };
            Ok(())
        }
        else {
            Err(alloc::format!("EFI ERROR: {}", status).to_string())
        }
    }

    else {
        let status = block_io_protocol.read_blocks(lba, count, buffer);
        if status == 0 {
            Ok(())
        }
        else {
            Err(alloc::format!("EFI ERROR: {}", status).to_string())
        }
    }
}







/// Searches for block devices and returns a Vector of EFIBlockDevice structs
fn probe_disks() -> Vec<EFIBlockDevice> {
    /* 
     * The locate_handle() function from the UEFI boot services table places a pointer in 'buffer' to an array of handles supporting the protocol that is being searched for. We obviously don't know how big the array is at compile time but fortunately if you call the function with
     * buffer_size set to 0 it will change buffer_size the size needed to hold the array. We can then create a vector with that value to hold the array of handles.
     */
    let mut buffer_size = 0;

    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, core::ptr::dangling_mut());
    let handles: Vec<usize> = vec![0; buffer_size / size_of::<usize>()];
    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, handles.as_ptr().cast_mut());

    
    // Iterate through the handles, parse the device path and add each disks to a vector
    let mut entries: Vec<EFIBlockDevice> = Vec::new();
    let mut hid = 0;


    for i in 0..handles.len() {
        // Get the device path protocol (First node in the path)
        let device_path_protocol_ptr: *mut *mut DevicePathProtocol = core::ptr::dangling_mut();
        BootServices::handle_protocol(handles[i] as *const usize, &(DevicePathProtocol::guid()), device_path_protocol_ptr.cast());
        
        let mut node: &DevicePathProtocol = unsafe { &mut (**device_path_protocol_ptr)};
        

        // Traverse the device path
        let mut new_device = false;
        // EFI Device path structure: ACPI->Hardware Device->Messaging Device->Media Device (if applicable)
        loop {
            match (node._type, node.subtype, node.length[0] + node.length[1]) {
                /* ACPI Device Path Node */
                (0x02, 1, 12) => {
                    #[allow(invalid_reference_casting)]
                    let acpi: &ACPIDevicePath = unsafe { &*((node as *const DevicePathProtocol).cast()) };

                    // new device?
                    if hid != acpi.hid {
                        new_device = true;
                        hid = acpi.hid;
                    }
                }

                _ => {}
            }

            /* Hardware device paths */
            loop {   
                node = node.next();
                match (node._type, node.subtype, node.length[0] + node.length[1]) {
                    // PCI Device path
                    (0x01, 0x01, 0x06) => { /* Do nothing */ }

                    _ => { panic!("No PCI bus found for storage device."); }
                }

                /* Messaging Device Paths */
                loop {
                    node = node.next();
                    match (node._type, node.subtype, node.length[0] + node.length[1]) {
                        // SATA device path node
                        (0x03, 18, 10) => {
                            if new_device {
                                // create new device
                                entries.push(EFIBlockDevice::new(name_device(DeviceType::HDD, &entries).as_str(), handles[i] as *const usize, false));
                                println!("Created block descriptor: {} with handle: 0x{:02X}", entries.last().unwrap().name, handles[i]);
                                break;
                            }
                        }

                        /* Last node / End of device path */
                        (0x7F, 0xFF | 0x01, 4) => { break; }

                        _ => { 
                            println!("Warning: Unknown storage device detected. Ignoring.");
                            break;
                        }
                    }


                    /* Media Device Paths */
                    loop {
                        node = node.next();
                        match (node._type, node.subtype, node.length[0] + node.length[1]) {
                            // Hard disk device path
                            (0x04, 1, 42) => {
                                // Create a new slice
                                let name = name_slice(DeviceType::HDD, &entries);
                                entries.push(EFIBlockDevice::new(name.as_str(), handles[i] as *const usize, true));
                                println!("Created block descriptor: {} with handle: 0x{:02X}", entries.last().unwrap().name, handles[i]);
                            }

                            /* Last node / End of device path */
                            (0x7F, 0xFF | 0x01, 4) => { break; }

                            _ => {}
                        }
                        break;
                    }
                    break;
                }
                break;
            }
            break;
        }
    }

    entries
}

enum DeviceType {
    HDD,
    Removable
}

fn name_device(device_type: DeviceType, devices: &Vec<EFIBlockDevice> ) -> String {
    let mut available = true;
    let mut disk_num = 0;

    loop {
        let name = {
            match device_type {
                DeviceType::HDD => { String::from(alloc::format!("disk{}", disk_num)) }

                _ => { panic!("No naming mechanism for selected device type") }
            }
        };

        for dev in devices.iter() {
            available = true;
            if dev.name == name {
                available = false;
            }
        }

        if available {
            return name
        }

        disk_num+=1;
    }
}

fn find_device(name: &str) -> Result<&'static EFIBlockDevice, ()> {
    unsafe {
        if !EFI_BLOCK_DEVICES.is_empty() {
            for device in EFI_BLOCK_DEVICES.iter() {
                if device.name == name {
                    return Ok(device)
                }
            }
        }
    }

    Err(())
}

fn name_slice(device_type: DeviceType, devices: &Vec<EFIBlockDevice> ) -> String {
    let mut slice_num = 1;

    let mut disk = &String::new();    
    for dev in devices.iter().rev() {
        if !dev.is_slice {
            disk = &dev.name;
            break;
        }
    }

    loop {
        let mut available = true;
        let name = {
            match device_type {
                DeviceType::HDD => { String::from(alloc::format!("{}s{}", disk, slice_num)) }

                _ => { panic!("No naming mechanism for selected device type") }
            }
        };

        for dev in devices.iter() {
            if dev.name.eq(&name) {
                available = false;
            }
        }

        if available {
            return name
        }
        slice_num+=1;
    }
}

/// Returns the name of the slice containing the EFI System Partition
pub fn find_efi_slice() -> Result<&'static str, String> {
    let loaded_image_protocol: *mut *mut LoadedImageProtocol = core::ptr::dangling_mut();
    BootServices::handle_protocol(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst), &(LoadedImageProtocol::guid()), loaded_image_protocol.cast());
    let loaded_image_protocol: &LoadedImageProtocol = unsafe { &mut (**loaded_image_protocol)};

    for dev in unsafe { EFI_BLOCK_DEVICES.iter() } {
        if dev.handle == loaded_image_protocol.device_handle {
            return Ok(dev.name.as_str())
        }
    }

    Err("Could not find EFI System Partition slice.".to_string())
}


pub fn init() -> Result<(), String>{
    unsafe { 
        if EFI_BLOCK_DEVICES.is_empty() { 
            EFI_BLOCK_DEVICES = probe_disks();
            Ok(())
        }
        else {
            Err("UEFI disk driver cannot be initialized more than once.".to_string())
        }
    }
}