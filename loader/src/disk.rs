// disk.rs

use crate::uefi;
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


    // for disk in entries {
    //     let guid = BlockIOProtocol::guid();
    //     let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::NonNull::<BlockIOProtocol>::dangling().as_ptr() as *mut *mut BlockIOProtocol;
    //     uefi::BootServices::handle_protocol(disk.handle, &guid as *const GUID, block_io_protocol as *mut *mut usize);
    //     let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };


    //     let buffer: Vec<u8> = vec![0; 1024];
    //     block_io_protocol.read_blocks(0, 1024, buffer.as_ptr() as *const usize);
    //     unsafe { hexdump_blocks!(1024, 8, 512, buffer.as_ptr()); }
    // }
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

    
    // Iterate through the handles and add each disks to a vector
    let mut entries: Vec<DiskEntry> = Vec::new();

    for (index, entry) in handles.iter().enumerate() {
        let entry = entry as *const usize;

        let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::NonNull::<BlockIOProtocol>::dangling().as_ptr() as *mut *mut BlockIOProtocol;
        BootServices::handle_protocol(handles[index] as *const usize, &(BlockIOProtocol::guid()), block_io_protocol as *mut *mut usize);

        let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };

        // Check if it's a hard disk
        if unsafe { !(*block_io_protocol.media).logical_partition && (*block_io_protocol.media).media_present } {
            let mut name = String::new();
            name.push_str(alloc::format!("/dev/disk{}", index ).as_str());
            println!("Found disk: {} with handle: 0x{:02X}", name, entry as usize);
            entries.push(DiskEntry::new(name.as_str(), handles[index] as *const usize));
        }

        // Check if it's removable storage
        if unsafe { !(*block_io_protocol.media).logical_partition && (*block_io_protocol.media).removable_media && (*block_io_protocol.media).media_present } {
            println!("Found removable disk: /dev/{} with handle: 0x{:02X}", "sdb1", entry as usize);
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