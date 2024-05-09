/*  disk.rs - UEFI disk driver
 *
 *  zOS  --  Advanced *NIX System
 *  Copyright (C) 2024  Free Software Foundation, Inc.
 *
 *  zOS is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  zOS is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with zOS. If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(dead_code)]

use core::ptr;
use alloc::vec;
use alloc::{string::{String, ToString}, vec::Vec};
use super::libuefi::{bootservices::BootServices, protocol::{block_io::BlockIOProtocol, device_path::{DevicePathProtocol, HardDriveDevicePath}}};
use crate::uuid::GUID;


static mut DISK_SLICE_INFO: Vec<DiskSliceInfo> = Vec::new();

#[derive(Clone, Copy)]
struct DiskSliceInfo {
    guid:                   GUID,
    handle:                 *const usize,
}

impl DiskSliceInfo {
    pub const fn new(guid: GUID, handle: *const usize) -> Self {
        Self {
            guid,
            handle,
        }
    }
}


/// Fills SLICE_ENTRIES with a list of slice GUIDs and their EFI Handle
pub fn init() {
    // Get a list of handles that support the BlockIOProtocol. This list includes every storage media device + their partitions.
    let handles = BootServices::locate_handle_by_protocol::<BlockIOProtocol>();
    
    // Iterate through the handles, looking specifically for the Hard Drive Device Path node. This specific node indicates that the handle belongs to a slice
    let mut partition_entries: Vec<DiskSliceInfo> = Vec::new();

    for i in 0..handles.len() {
        let mut node = BootServices::handle_protocol::<DevicePathProtocol>(handles[i] as *const usize);

        while (node._type, node.subtype) != (0x7F, 0xFF)  {
            match (node._type, node.subtype, node.length[0] + node.length[1]) {
                // Hard drive device path
                (4, 1, 42) => {
                    // Cast the current node as HardDriveDevicePath so we can read the GUID
                    #[allow(invalid_reference_casting)]
                    let hddp: &HardDriveDevicePath = unsafe { &*((node as *const DevicePathProtocol).cast()) };
                    let guid = hddp.partition_sig;
            
                    partition_entries.push(DiskSliceInfo::new(guid, handles[i] as *const usize));
                }

                _ => {}
            }

            node = node.next();
        }
    }


    assert!(partition_entries.is_empty() == false);
    unsafe { DISK_SLICE_INFO = partition_entries; }
}




pub unsafe fn read_bytes_raw(slice: GUID, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    //
    // TODO: determine if we should use u64 or usize
    //
    let count = count as u64;
    
    let phys_block_size = get_phys_block_size(slice);

    // if 'count' is equal to an even multiple of physical blocks we can simply read the blocks into buffer
    //
    // if 'count' is NOT equal to an even multiple of physical blocks. We can read 'full_count' number of blocks into buffer, then create a temporary buffer with a size equal to phys_block_size to hold the full block
    // We can then copy the bytes we actually need into the buffer and return
    if (count % phys_block_size) == 0 {
        return read_blocks(slice, lba, count, buffer);
    }
    else {
        let full_count = (count / phys_block_size) * phys_block_size;
        let rem = count % phys_block_size;

        // Read the amount of blocks that fit into 'count' evenly into the buffer
        read_blocks(slice, lba, full_count, buffer).unwrap();

        // Create a temporary buffer = to size of one block
        let buff_size: usize = phys_block_size.try_into().unwrap();
        let mut tmp: Vec<u8> = vec![0; buff_size];


        // Read the remainder of bytes into the temporary buffer
        read_blocks(slice, lba, phys_block_size, tmp.as_mut_ptr().cast()).unwrap();

        // Copy 'remainder' into 'buffer'
        unsafe {
            let rem_ptr = {
                let buff_ptr = buffer.as_mut().unwrap() as *mut u8;
                buff_ptr.offset(full_count as isize)
            };

            ptr::copy(tmp.as_ptr(), rem_ptr, rem as usize);
        }
    }


    return Ok(())
}




pub unsafe fn read_blocks(guid: GUID, lba: u64, buffer_size: u64, buffer: *mut u8) -> Result<(), String> {
    let block_io_protocol = BootServices::handle_protocol::<BlockIOProtocol>(lookup_handle(guid));
    let block_size = (*block_io_protocol.media).block_size as usize;

    assert_eq!(buffer_size % block_size as u64, 0, "'buffer_size' must be a multiple of the disk's physical block size. (e.g 512 bytes)");

    let status = block_io_protocol.read_blocks(lba, buffer_size as usize, buffer);
    if status == 0 {
        Ok(())
    }

    else {
        Err(alloc::format!("EFI ERROR: {}", status).to_string())
    }
    
}




pub fn get_phys_block_size(slice: GUID) -> u64 {
    let block_io_protocol = BootServices::handle_protocol::<BlockIOProtocol>(lookup_handle(slice));
    
    unsafe { (*block_io_protocol.media).block_size as u64 }
}




/// Returns the EFI_HANDLE belonging to a given slice
fn lookup_handle(guid: GUID) -> *const usize {
    unsafe {
        assert_eq!(DISK_SLICE_INFO.is_empty(), false);
        
        for partition in DISK_SLICE_INFO.iter() {
            if partition.guid.as_string() == guid.as_string() {
                return partition.handle
            }
        }
    }

    panic!("Partition not found: {}", guid.as_string());
}



/// Finds a SliceEntry by GUID
fn find_slice(guid: GUID) -> DiskSliceInfo {
    unsafe {
        assert_eq!(DISK_SLICE_INFO.is_empty(), false);
        
        for partition in DISK_SLICE_INFO.iter() {
            if partition.guid == guid {
                return *partition
            }
        }

        panic!("Could not find slice with GUID: {}.", guid.as_string());
    }
}
