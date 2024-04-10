// disk.rs

use crate::uefi::{self, ACPIDevicePath, DevicePathProtocol};
use alloc::slice;
use uefi::{LocateSearchType, BootServices, BlockIOProtocol};
use core::ptr;
use core::mem::size_of;
use alloc::{vec, vec::Vec};
use alloc::string::{String, ToString};

struct EFIBlockDevice {
    name: String,
    handle: *const usize,
    is_slice: bool,
    slice_number: u16,
   // hid:    usize,
}

impl EFIBlockDevice {
    pub fn new(name: &str, handle: *const usize, is_slice: bool, slice_number: u16) -> Self {
        let name = name.to_string();
        Self {
            name,
            handle,
            is_slice,
            slice_number
        }
    }
}



pub fn read_blocks() {
    let entries = probe_disks();


    for disk in entries {
        if disk.name.cmp(&String::from("/dev/disk0s1")).is_eq() {
            let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::NonNull::<BlockIOProtocol>::dangling().as_ptr() as *mut *mut BlockIOProtocol;
            uefi::BootServices::handle_protocol(disk.handle, &(BlockIOProtocol::guid()), block_io_protocol as *mut *mut usize);
            let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };


            let buffer: Vec<u8> = vec![0; 512];
            block_io_protocol.read_blocks(2, 1024, buffer.as_ptr() as *const usize);


            let mut ptr = buffer.as_ptr() as usize;
            ptr += 56;
            let magic: u16 = unsafe { *(ptr as *const u16) as u16};
            println!("EXT2 magic: 0x{:X}", magic);

            // use debugutils::hexdump_blocks;
            // unsafe { hexdump_blocks!(512, 8, 512, buffer.as_ptr()); }
        }
        
    }
}


/// Searches for block devices and returns a Vector of EFIBlockDevice structs
fn probe_disks() -> Vec<EFIBlockDevice> {
    /* The locate_handle() function from the UEFI boot services table places a pointer in 'buffer' to an array of handles supporting the protocol that is being searched for. We obviously don't know how big the array is at compile time but fortunately if you call the function with
     * buffer_size set to 0 it will change buffer_size the size needed to hold the array. We can then create a vector with that value to hold the array of handles.
     */
    let mut buffer_size = 0;

    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, core::ptr::NonNull::<usize>::dangling().as_ptr());
    let handles: Vec<usize> = vec![0; buffer_size / size_of::<usize>()];
    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, handles.as_ptr() as *mut usize);

    
    // Iterate through the handles, parse the device path and add each disks to a vector
    let mut entries: Vec<EFIBlockDevice> = Vec::new();
    let mut hid = 0;


    for i in 0..handles.len() {
        // Get the device path protocol (First node in the path)
        let device_path_protocol_ptr: *mut *mut DevicePathProtocol = core::ptr::NonNull::<DevicePathProtocol>::dangling().as_ptr() as *mut *mut DevicePathProtocol;
        BootServices::handle_protocol(handles[i] as *const usize, &(DevicePathProtocol::guid()), device_path_protocol_ptr as *mut *mut usize);
        
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
                                entries.push(EFIBlockDevice::new(name_device(DeviceType::hdd, &entries).as_str(), handles[i] as *const usize, false, 0));
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
                                let (name, slice) = name_slice(DeviceType::hdd, &entries);
                                entries.push(EFIBlockDevice::new(name.as_str(), handles[i] as *const usize, true, slice));
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
    hdd,
    removable
}

fn name_device(device_type: DeviceType, devices: &Vec<EFIBlockDevice> ) -> String {
    let mut available = true;
    let mut disk_num = 0;

    loop {
        let name = {
            match device_type {
                DeviceType::hdd => { String::from(alloc::format!("disk{}", disk_num)) }

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

fn name_slice(device_type: DeviceType, devices: &Vec<EFIBlockDevice> ) -> (String, u16) {
    let mut available = true;
    let mut slice_num = 1;

    let mut disk = &String::new();    
    for dev in devices.iter().rev() {
        if !dev.is_slice {
            disk = &dev.name;
            break;
        }
    }


    loop {
        let name = {
            match device_type {
                DeviceType::hdd => { String::from(alloc::format!("{}s{}", disk, slice_num)) }

                _ => { panic!("No naming mechanism for selected device type") }
            }
        };

        for dev in devices.iter() {
            available = true;
            if dev.name.eq(&name) {
                available = false;
            }
        }

        if available {
            return (name, slice_num)
        }
        slice_num+=1;
    }
}



fn _find_boot_disk() {
    //
}

pub fn _initialize() {
    //
}