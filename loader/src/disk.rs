// disk.rs

use crate::uefi::{self, ACPIDevicePath, DevicePathProtocol};
use uefi::{LocateSearchType, BootServices, BlockIOProtocol};
use core::ptr;
use core::mem::size_of;
use alloc::{vec, vec::Vec};
use alloc::string::{String, ToString};

struct EFIBlockDevice {
    name: String,
    // number
    handle: *const usize,
   // hid:    usize,
}

impl EFIBlockDevice {
    pub fn new(name: &str, handle: *const usize) -> Self {
        let name = name.to_string();
        Self {
            name,
            handle,
           // hid,
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

    let mut disk = 0;
    let mut slice = 1;
    let mut hid = 0;

    for i in 0..handles.len() {
        // Get the device path protocol
        let device_path_protocol_ptr: *mut *mut DevicePathProtocol = core::ptr::NonNull::<DevicePathProtocol>::dangling().as_ptr() as *mut *mut DevicePathProtocol;
        BootServices::handle_protocol(handles[i] as *const usize, &(DevicePathProtocol::guid()), device_path_protocol_ptr as *mut *mut usize);
        
        let mut node: &DevicePathProtocol = unsafe { &mut (**device_path_protocol_ptr)};
        

        // Traverse the device path
        let mut traversing = true;
        let mut new_device = false;
        // EFI Device path structure: ACPI->Hardware Device->Messaging Device->Media Device (if applicable)
        while traversing {
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
                    else {
                        new_device = false;
                    }
                }

                _ => {}
            }

            /* Hardware device paths */
            while traversing {   
                node = node.next();
                match (node._type, node.subtype, node.length[0] + node.length[1]) {
                    // PCI Device path
                    (0x01, 0x01, 0x06) => { /* Do nothing */ }

                    _ => { panic!("No PCI bus found for storage device."); }
                }

                /* Messaging Device Paths */
                while traversing {   
                    node = node.next();
                    match (node._type, node.subtype, node.length[0] + node.length[1]) {
                        // SATA device path node
                        (0x03, 18, 10) => {
                            if new_device {
                                // create new device
                                println!("Found a SATA device!");
                                disk += 1;
                                slice = 1;
                                let mut name = String::new();
                                name.push_str(alloc::format!("/dev/disk{}", disk ).as_str());
                                println!("Created block descriptor: {} with handle: 0x{:02X}", name, handles[i]);
                                entries.push(EFIBlockDevice::new(name.as_str(), handles[i] as *const usize));

                                traversing = false;
                                break;
                            }
                        }

                        /* Last node / End of device path */
                        (0x7F, 0xFF | 0x01, 4) => {
                            traversing = false;
                            break;
                        }

                        _ => { println!("Warning: Unknown storage device detected. Ignoring."); }
                    }


                    /* Media Device Paths */
                    while traversing {
                        node = node.next();
                        match (node._type, node.subtype, node.length[0] + node.length[1]) {
                            // Hard disk device path
                            (0x04, 1, 42) => {
                                // Create a new slice
                                let mut name = String::new();
                                name.push_str(alloc::format!("/dev/disk{}s{}", disk, slice).as_str());
                                println!("Created block descriptor: {} with handle: 0x{:02X}", name, handles[i]);
                                entries.push(EFIBlockDevice::new(name.as_str(), handles[i] as *const usize));
                                slice += 1;
                            }

                            /* Last node / End of device path */
                            (0x7F, 0xFF | 0x01, 4) => {
                                traversing = false;
                                break;
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
    }

    entries
}

fn _find_boot_disk() {
    //
}

pub fn _initialize() {
    //
}