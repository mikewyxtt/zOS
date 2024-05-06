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


use alloc::{string::{String, ToString}, vec::Vec};
use crate::{extfs, fat, libuefi::{bootservices::BootServices, protocol::{device_path::{DevicePathProtocol, HardDriveDevicePath}, loaded_image::LoadedImageProtocol}, GUID}};

pub const EOF: i8 = -1;



#[derive(Clone, Copy, PartialEq, Eq)]
enum FilesystemType {
    FAT,
    EXT,
    UNKNOWN,
}


#[derive(PartialEq)]
pub struct File {
    pub slice:              GUID,
    pub path:               String,
    pub filesize:           u64,
    _read_raw:              unsafe fn(&Self, usize, *mut u8),
}

enum DriverInfo {
    FAT {
        first_cluster_number:   u32,
        clusters:               Vec<u32>,
    }
}


impl File {
    pub fn new(slice: GUID, path: &str, filesize: u64, read_raw: unsafe fn(&Self, usize, *mut u8)) -> Self {
        Self {
            slice,
            path: path.to_string(),
            filesize,
            _read_raw:  read_raw,
        }
    }

    /// Opens a file, reading its entire contents into a vector
    pub fn open(slice: GUID, path: &str) -> Self {
        match detect_fs_type(slice) {
            FilesystemType::FAT => {
                return fat::open(slice, path);
            }

            FilesystemType::EXT => {
                return extfs::open(slice, path);
            }

            FilesystemType::UNKNOWN => {
                panic!("Trying to open \"{path}\" on slice with GUID {} failed: Unknown filesystem \nHalting.", slice.as_string())
            }
        }
    }

    /// Reads 'count' bytes from the file into 'buffer'. Returns the amount of bytes that were read
    pub unsafe fn read_raw(&self, count: usize, buffer: *mut u8) -> usize {
        assert!(count <= self.filesize as usize);
        
        (self._read_raw)(&self, count, buffer);

        count
    }

    /// Reads the entire contents of the file into a String
    pub fn read_to_string(&self) -> String {
        let mut s = String::new();

        let mut contents = Vec::with_capacity(self.filesize as usize);
        unsafe {
            self.read_raw(self.filesize as usize, contents.as_mut_ptr());
            contents.set_len(self.filesize as usize);
        }

        for b in &contents {
            s.push(*b as char);
        }

        s
    }
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




/// Start the filesystem driver
pub fn start() {
    // Do something
}