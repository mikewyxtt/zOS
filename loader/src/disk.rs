// disk.rs

use crate::uefi::{self, DevicePathProtocol};
use uefi::{LocateSearchType, BootServices, BlockIOProtocol};
use core::ptr;
use core::mem::size_of;
use alloc::{vec, vec::Vec};
use alloc::string::{String, ToString};

struct DiskEntry {
    name: String,
    handle: *const usize
}

impl DiskEntry {
    pub fn new(name: &str, handle: *const usize) -> Self {
        let name = name.to_string();
        Self {
            name,
            handle,
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

/// Searches for block devices and returns a Vector of DiskEntry structs
fn probe_disks() -> Vec<DiskEntry> {
    /* The locate_handle() function from the UEFI boot services table places a pointer in 'buffer' to an array of handles supporting the protocol that is being searched for. We obviously don't know how big the array is at compile time but fortunately if you call the function with
     * buffer_size set to 0 it will change buffer_size the size needed to hold the array. We can then create a vector with that value to hold the array of handles.
     */
    let mut buffer_size = 0;

    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, core::ptr::NonNull::<usize>::dangling().as_ptr());
    let handles: Vec<usize> = vec![0; buffer_size / size_of::<usize>()];
    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, handles.as_ptr() as *mut usize);

    
    // Iterate through the handles, parse the device path and add each disks to a vector
    let mut entries: Vec<DiskEntry> = Vec::new();

    let mut disk = 0;

    for i in 0..handles.len() {
        // Get the device path protocol
        let device_path: *mut *mut DevicePathProtocol = core::ptr::NonNull::<DevicePathProtocol>::dangling().as_ptr() as *mut *mut DevicePathProtocol;
        BootServices::handle_protocol(handles[i] as *const usize, &(DevicePathProtocol::guid()), device_path as *mut *mut usize);
        
        let mut device_path: &DevicePathProtocol = unsafe { &mut (**device_path)};

        while device_path._type != 0x7F {
            // are we on the last node?
            let next = unsafe { device_path.next() };
            
            if next._type == 0x7F {
                // Is this a SATA drive?
                if device_path.subtype == 18 {
                    let mut name = String::new();
                    name.push_str(alloc::format!("/dev/disk{}", disk ).as_str());
                    println!("Found block device: {} with handle: 0x{:02X}", name, handles[i]);
                    entries.push(DiskEntry::new(name.as_str(), handles[i] as *const usize));
                    disk += 1;
                }

                // is this a partition?
                if device_path.subtype == 4 {
                    //
                }
            }

            device_path = unsafe { device_path.next() };
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