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
use crate::libuefi::{bootservices::BootServices, protocol::{block_io::BlockIOProtocol, device_path::{DevicePathProtocol, HardDriveDevicePath}}, GUID};


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





pub unsafe fn read_bytes_raw(guid: GUID, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {

    let block_io_protocol = BootServices::handle_protocol::<BlockIOProtocol>(lookup_handle(guid));

    let block_size = (*block_io_protocol.media).block_size as usize;

    if count < block_size {
        let mut tmp: Vec<u8> = vec![0; block_size];
        let status = block_io_protocol.read_blocks(lba, block_size, tmp.as_mut_ptr());
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
