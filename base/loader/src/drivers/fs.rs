/*  fs.rs - Basic filesystem driver
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


use alloc::{string::{String, ToString}, vec, vec::Vec};
use crate::{extfs, fat, libuefi::{bootservices::BootServices, protocol::{device_path::{DevicePathProtocol, HardDriveDevicePath}, loaded_image::LoadedImageProtocol}, GUID}};



#[derive(Clone, Copy, PartialEq, Eq)]
enum FilesystemType {
    FAT,
    EXT,
    UNKNOWN,
}




/// Returns the GUID partition signature of the ESP
pub fn get_esp_guid() -> GUID {
    let handle = BootServices::handle_protocol::<LoadedImageProtocol>(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst)).device_handle;
    let mut node = BootServices::handle_protocol::<DevicePathProtocol>(handle as *const usize);

    while (node._type, node.subtype) != (0x7F, 0xFF)  {
                match (node._type, node.subtype, node.length[0] + node.length[1]) {
                    // Hard drive device path
                    (4, 1, 42) => {
                        // Cast the current node as HardDriveDevicePath so we can read the GUID
                        #[allow(invalid_reference_casting)]
                        let hddp: &HardDriveDevicePath = unsafe { &*((node as *const DevicePathProtocol).cast()) };
                        let guid = hddp.partition_sig;

                        return guid;
                    }
    
                    _ => {}
                }
    
                node = node.next();
            }

    panic!("FS error: Could not find EFI System Partition. Halting.");
}



/// Detects the filesystem of the slice
fn detect_fs_type(guid: GUID) -> FilesystemType {
    if extfs::detect(guid) {
        return FilesystemType::EXT;
    }
    else if fat::detect(guid) {
        return FilesystemType::FAT;
    }
    else {
        return FilesystemType::UNKNOWN;
    }
}



/// Reads a files entire contents into *buffer*
///
/// If *buffer* is a null ptr, this fn returns the buffer size needed to contain the file. Otherwise, it returns None.
pub unsafe fn read_file_raw(slice: GUID, path: &str, buffer: *mut u8) -> Option<u64> {
    match detect_fs_type(slice) {
        FilesystemType::FAT => {
            return fat::read_bytes_raw(slice, path, buffer);
        }

        FilesystemType::EXT => {
            // return extfs::read_bytes_raw(slice, path, buffer);
            return None;
        }

        FilesystemType::UNKNOWN => {
            panic!("Trying to open \"{path}\" on slice with GUID {} failed: Unknown filesystem \nHalting.", slice.as_string())
        }
    }
}



/// Reads the entire contents of the file into a String
pub fn read_to_string(slice: GUID, path: &str) -> String {
    let filesize = unsafe { read_file_raw(slice, path, core::ptr::null_mut()).unwrap() };
    let contents: Vec<u8> = { 
        let mut buffer: Vec<u8> = vec![0; filesize.try_into().unwrap()];
        unsafe { read_file_raw(slice, path, buffer.as_mut_ptr()) };

        buffer
    };

    let mut s = String::new();

    for b in &contents {
        s.push(*b as char);
    }

    s
}



/// Start the filesystem driver
pub fn start() {
    // Do something
}


// Generic filetype, used inside all other file types
pub struct File {
    slice:  GUID,
    uuid:   Option<u128>,
    path:   String,
}

impl File {
    pub fn open_by_guid(slice: GUID, path: &str) -> Self {
        Self {
            slice:  slice,
            uuid:   None,
            path:   path.to_string(),
        }
    }

    pub fn open_by_uuid(uuid: u128, path: &str) -> Self {
        let slice = get_esp_guid();

        Self {
            slice:  slice,
            uuid:   Some(uuid),
            path:   path.to_string(),
        }
    }

    /// Reads a files entire contents into *buffer*
    ///
    /// If *buffer* is a null ptr, this fn returns the buffer size needed to contain the file. Otherwise, it returns None.
    pub unsafe fn read_raw(&self, buffer: *mut u8) -> Option<u64> {
        match detect_fs_type(self.slice) {
            FilesystemType::FAT => {
                return fat::read_bytes_raw(self.slice, &self.path, buffer);
            }

            FilesystemType::EXT => {
                // return extfs::read_bytes_raw(slice, path, buffer);
                return None;
            }

            FilesystemType::UNKNOWN => {
                let path = &self.path;
                if self.uuid.is_some() {
                    panic!("Trying to open \"{path}\" on slice with UUID {} failed: Unknown filesystem \nHalting.", self.uuid.unwrap())
                }
                else {
                    panic!("Trying to open \"{path}\" on slice with GUID {} failed: Unknown filesystem \nHalting.", self.slice.as_string())
                }
            }
        }
    }

    /// Reads the entire contents of the file into a String
    pub fn read_to_string(&self) -> Result<String, ()> {
        let filesize = unsafe { self.read_raw(core::ptr::null_mut()).unwrap() };
        let contents: Vec<u8> = { 
            let mut buffer: Vec<u8> = vec![0; filesize.try_into().unwrap()];
            unsafe { self.read_raw(buffer.as_mut_ptr()) };

            buffer
        };

        let mut s = String::new();

        for b in &contents {
            s.push(*b as char);
        }

        Ok(s)
    }
}



// Firwmare has everything needed to load files and print to screen, no drivers needed
mod firmware {
    //
    pub mod disk {
        pub struct Disk {
            pub block_size: u32,
        }
    }

    pub mod console {
        pub fn clear() {
            //
        }

        pub fn _print() {
            //
        }
    }

    pub mod arch {
        use crate::libuefi::{bootservices::BootServices, protocol::{device_path::{DevicePathProtocol, HardDriveDevicePath}, loaded_image::LoadedImageProtocol}, GUID};

        pub fn get_esp_guid() -> GUID {
            let handle = BootServices::handle_protocol::<LoadedImageProtocol>(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst)).device_handle;
            let mut node = BootServices::handle_protocol::<DevicePathProtocol>(handle as *const usize);
        
            while (node._type, node.subtype) != (0x7F, 0xFF)  {
                        match (node._type, node.subtype, node.length[0] + node.length[1]) {
                            // Hard drive device path
                            (4, 1, 42) => {
                                // Cast the current node as HardDriveDevicePath so we can read the GUID
                                #[allow(invalid_reference_casting)]
                                let hddp: &HardDriveDevicePath = unsafe { &*((node as *const DevicePathProtocol).cast()) };
                                let guid = hddp.partition_sig;
        
                                return guid;
                            }
            
                            _ => {}
                        }
            
                        node = node.next();
                    }
        
            panic!("FS error: Could not find EFI System Partition. Halting.");
        }
    }
}
