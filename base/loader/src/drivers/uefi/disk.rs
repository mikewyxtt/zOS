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

use core::{mem::size_of, ptr};
use alloc::{string::{String, ToString}, vec, vec::Vec};
use crate::libuefi::{bootservices::{BootServices, LocateSearchType}, protocol::{block_io::BlockIOProtocol, device_path::{DevicePathProtocol, HardDriveDevicePath}, file::{File, FileInfo}, filesystem::SimpleFilesystem, loaded_image::LoadedImageProtocol}, GUID};


static mut PARTITION_ENTRIES: Vec<PartitionEntry> = Vec::new();


struct PartitionEntry {
    guid: GUID,
    handle: *const usize,
}

impl PartitionEntry {
    pub const fn new(guid: GUID, handle: *const usize) -> Self {
        Self {
            guid,
            handle
        }
    }
}


/// Fills PARTITION_ENTRIES with a list of partition GUIDs and their EFI Handle
pub fn probe_disks() {
    /* 
     * The locate_handle() function from the UEFI boot services table places a pointer in 'buffer' to an array of handles supporting the protocol that is being searched for. We obviously don't know how big the array is at compile time but fortunately if you call the function with
     * buffer_size set to 0 it will change buffer_size the size needed to hold the array. We can then create a vector with that value to hold the array of handles.
     */
    let mut buffer_size = 0;

    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, core::ptr::dangling_mut());
    let handles: Vec<usize> = vec![0; buffer_size / size_of::<usize>()];
    BootServices::locate_handle(LocateSearchType::ByProtocol, &(BlockIOProtocol::guid()), ptr::null(), &mut buffer_size, handles.as_ptr().cast_mut());

    
    // Iterate through the handles, parse the device path and add each disks to a vector
    let mut partition_entries: Vec<PartitionEntry> = Vec::new();

    for i in 0..handles.len() {
        let device_path_protocol_ptr: *mut *mut DevicePathProtocol = core::ptr::dangling_mut();
        BootServices::handle_protocol(handles[i] as *const usize, &(DevicePathProtocol::guid()), device_path_protocol_ptr.cast());
        
        let mut node: &DevicePathProtocol = unsafe { &mut (**device_path_protocol_ptr)};

        while (node._type, node.subtype) != (0x7F, 0xFF)  {
            match (node._type, node.subtype, node.length[0] + node.length[1]) {
                // Hard drive device path
                (4, 1, 42) => {
                    // Cast the current node as HardDriveDevicePath so we can read the GUID
                    #[allow(invalid_reference_casting)]
                    let hddp: &HardDriveDevicePath = unsafe { &*((node as *const DevicePathProtocol).cast()) };
                    let guid = hddp.partition_sig;
            
                    partition_entries.push(PartitionEntry::new(guid, handles[i] as *const usize));
                    ldrprintln!("Found partition with GUID: {}", guid.as_string());
                }

                _ => {}
            }

            node = node.next();
        }
    }

    assert!(partition_entries.is_empty() == false);
    unsafe { PARTITION_ENTRIES = partition_entries; }
}





pub unsafe fn read_bytes_raw(guid: GUID, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {

    let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::dangling_mut();
    BootServices::handle_protocol(lookup_handle(guid), &(BlockIOProtocol::guid()), block_io_protocol.cast());
    let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };

    let block_size = unsafe { (*block_io_protocol.media).block_size } as usize;

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




fn lookup_handle(guid: GUID) -> *const usize {
    unsafe {
        assert!(PARTITION_ENTRIES.is_empty() == false);
        
        for partition in PARTITION_ENTRIES.iter() {
            if partition.guid.as_string() == guid.as_string() {
                return partition.handle
            }
        }
    }

    panic!("Partition not found: {}", guid.as_string());
}


pub fn getcfg() {
    let loaded_image_protocol: *mut *mut LoadedImageProtocol = core::ptr::dangling_mut();
    BootServices::handle_protocol(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst), &(LoadedImageProtocol::guid()), loaded_image_protocol.cast());
    let loaded_image_protocol: &LoadedImageProtocol = unsafe { &mut (**loaded_image_protocol)};

    let filesys_protocol: *mut *mut SimpleFilesystem = core::ptr::dangling_mut();
    BootServices::handle_protocol(loaded_image_protocol.device_handle, &(SimpleFilesystem::guid()), filesys_protocol.cast());
    let filesys_protocol: &SimpleFilesystem = unsafe { &mut (**filesys_protocol) };

    let file = filesys_protocol.open_volume();

    // let file = {
    //     let loaded_image_protocol: *mut *mut LoadedImageProtocol = core::ptr::dangling_mut();
    //     BootServices::handle_protocol(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst), &(LoadedImageProtocol::guid()), loaded_image_protocol.cast());
    //     let loaded_image_protocol: &LoadedImageProtocol = unsafe { &mut (**loaded_image_protocol)};

    //     let filesys_protocol: *mut *mut SimpleFilesystem = core::ptr::dangling_mut();
    //     BootServices::handle_protocol(loaded_image_protocol.device_handle, &(SimpleFilesystem::guid()), filesys_protocol.cast());
    //     let filesys_protocol: &SimpleFilesystem = unsafe { &mut (**filesys_protocol)};

    //     filesys_protocol.open_volume()
    // };
    

    let file = file.open("\\EFI\\BOOT\\ZOS\\LOADER.CFG\0", 1, None);
    let info = file.get_info(FileInfo::guid());
    ldrprintln!("file size: {} bytes", info.file_size);

    let contents = file.read();

    for b in contents {
        let c = b as char;
        ldrprint!("{}", c);
    }
    ldrprint!("\n");

}
