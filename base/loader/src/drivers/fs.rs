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
use crate::{extfs, fat};
use crate::uuid::GUID;



#[derive(Clone, Copy, PartialEq, Eq)]
enum FilesystemType {
    FAT,
    EXT,
    UNKNOWN,
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


// Generic filetype, used inside all other file types
pub struct File {
    slice:  GUID,
    path:   String,
}

impl File {
    pub fn open_by_guid(slice: GUID, path: &str) -> Self {
        Self {
            slice:  slice,
            path:   path.to_string(),
        }
    }

    pub fn open_by_uuid(_uuid: u128, _path: &str) -> Self {
        todo!();
        // let slice: GUID = unsafe { core::mem::zeroed() };

        // Self {
        //     slice:  slice,
        //     uuid:   Some(uuid),
        //     path:   path.to_string(),
        // }
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
                return extfs::read_bytes_raw(self.slice, &self.path, buffer);
                // return extfs::read_bytes_raw(self.slice, &self.path, buffer).unwrap();
            }

            FilesystemType::UNKNOWN => {
                // let path = &self.path;
                // if self.uuid.is_some() {
                //     panic!("Trying to open \"{path}\" on slice with UUID {} failed: Unknown filesystem \nHalting.", self.uuid.unwrap())
                // }
                // else {
                //     panic!("Trying to open \"{path}\" on slice with GUID {} failed: Unknown filesystem \nHalting.", self.slice.as_string())
                // }

                let path = &self.path;
                panic!("Trying to open \"{path}\" on slice with GUID {} failed: Unknown filesystem \nHalting.", self.slice.as_string())
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
