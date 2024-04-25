use core::{mem::size_of, ptr};

use alloc::{string::{String, ToString}, vec, vec::Vec};
use crate::{disk::BlockDevice, libuefi::{bootservices::{BootServices, LocateSearchType}, protocol::{block_io::BlockIOProtocol, device_path::{ACPIDevicePath, DevicePathProtocol}}}};

static mut EFI_DEVICE_INFO: Vec<EFIDeviceInfo> = Vec::new();

const BLOCK_SIZE: u16 = 512;


struct EFIDeviceInfo {
    name: String,
    handle: *const usize,
}

impl EFIDeviceInfo {
    pub fn new(name: String, handle: *const usize) -> Self {
        Self {
            name,
            handle
        }
    }
}



/// Searches for block devices and returns a Vector of EFIBlockDevice structs
pub fn probe_disks() -> Vec<BlockDevice> {
    /* 
     * The locate_handle() function from the UEFI boot services table places a pointer in 'buffer' to an array of handles supporting the protocol that is being searched for. We obviously don't know how big the array is at compile time but fortunately if you call the function with
     * buffer_size set to 0 it will change buffer_size the size needed to hold the array. We can then create a vector with that value to hold the array of handles.
     */
    let mut buffer_size = 0;

    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, core::ptr::dangling_mut());
    let handles: Vec<usize> = vec![0; buffer_size / size_of::<usize>()];
    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, handles.as_ptr().cast_mut());

    
    // Iterate through the handles, parse the device path and add each disks to a vector
    let mut device_entries: Vec<BlockDevice> = Vec::new();
    let mut efi_entries: Vec<EFIDeviceInfo> = Vec::new();
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
                                let name = crate::disk::name_device(false, false, &device_entries);
                                device_entries.push(BlockDevice::new(name.clone(), false, false, BLOCK_SIZE, read_bytes_raw));
                                efi_entries.push(EFIDeviceInfo::new(name.clone(), handles[i] as *const usize));
                                println!("Created block descriptor: {} with handle: 0x{:02X}", device_entries.last().unwrap().name, handles[i]);
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
                                let name = crate::disk::name_device(false, true,&device_entries);
                                device_entries.push(BlockDevice::new(name.clone(), true, false, BLOCK_SIZE, read_bytes_raw));
                                efi_entries.push(EFIDeviceInfo::new(name.clone(), handles[i] as *const usize));
                                println!("Created block descriptor: {} with handle: 0x{:02X}", device_entries.last().unwrap().name, handles[i]);
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
    unsafe { EFI_DEVICE_INFO = efi_entries; }
    device_entries
}






unsafe fn read_bytes_raw(dev: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::dangling_mut();
    BootServices::handle_protocol(lookup_handle(dev.to_string()), &(BlockIOProtocol::guid()), block_io_protocol.cast());
    let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };

    if count < BLOCK_SIZE.into() {
        let mut tmp: Vec<u8> = vec![0; BLOCK_SIZE.into()];
        let status = block_io_protocol.read_blocks(lba, BLOCK_SIZE.into(), tmp.as_mut_ptr());
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



fn lookup_handle(name: String) -> *const usize {
    unsafe {
        for device in EFI_DEVICE_INFO.iter() {
            if device.name == name {
                return device.handle
            }
        }
    }

    panic!("Device not found: {}", name);
}






// /// Returns the name of the slice containing the EFI System Partition
// pub fn find_efi_slice() -> Result<&'static str, String> {
//     let loaded_image_protocol: *mut *mut LoadedImageProtocol = core::ptr::dangling_mut();
//     BootServices::handle_protocol(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst), &(LoadedImageProtocol::guid()), loaded_image_protocol.cast());
//     let loaded_image_protocol: &LoadedImageProtocol = unsafe { &mut (**loaded_image_protocol)};

//     for dev in unsafe { EFI_BLOCK_DEVICES.iter() } {
//         if dev.handle == loaded_image_protocol.device_handle {
//             return Ok(dev.name.as_str())
//         }
//     }

//     Err("Could not find EFI System Partition slice.".to_string())
// }