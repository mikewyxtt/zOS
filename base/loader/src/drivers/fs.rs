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

use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::libuefi::{bootservices::BootServices, protocol::{block_io::BlockIOProtocol, device_path::{DevicePathProtocol, HardDriveDevicePath}, file::{self, EFI_File, FileInfo}, filesystem::SimpleFilesystem, loaded_image::LoadedImageProtocol}, GUID};


static mut SLICE_ENTRIES: Vec<SliceInfo> = Vec::new();

pub const EOF: i8 = -1;


#[derive(Clone, Copy, PartialEq, Eq)]
enum FilesystemType {
    FAT,
    XFS,
    UNKNOWN,
}



#[derive(Clone, Copy)]
struct SliceInfo {
    guid:       GUID,
    handle:     *const usize,
    fs_type:    FilesystemType,
    is_esp:     bool,
}

impl SliceInfo {
    pub const fn new(guid: GUID, handle: *const usize, fs_type: FilesystemType, is_esp: bool) -> Self {
        Self {
            guid,
            handle,
            fs_type,
            is_esp
        }
    }
}

#[derive(PartialEq)]
pub struct File {
    slice:              GUID,
    path:               String,
    filesize:           u64,
    position:           i64,
}


impl File {
    /// Opens a file, reading its entire contents into a vector
    pub fn open(slice: GUID, path: &str) -> Self {

        let slice = find_slice(slice);

        match slice.fs_type {
            FilesystemType::FAT => {
                // UEFI paths are Microsoft style '\' rather than '/'
                let path = String::from(format!("{}\0", path.to_string().to_ascii_uppercase().replace("/", "\\")));

                // First we need to access the filesystem protocol to open the root directory
                let filesys_protocol = BootServices::handle_protocol::<SimpleFilesystem>(slice.handle);
                let file = filesys_protocol.open_volume();
              
                // Now we can open the file
                let file = file.open(path.as_str(), 1, None);
                let info = file.get_info(FileInfo::guid());                

                

                // Close the EFI file, we will reopen it when we read from it..
                // !!Needs to be done!!

                return Self {
                    slice:      slice.guid,
                    path,
                    position: 0,
                    filesize: info.file_size,
                }
            }

            FilesystemType::XFS => {
                let filesize = 0;
                return Self {
                    slice:      slice.guid,
                    path: path.to_string(),
                    position: 0,
                    filesize: filesize,
                }
            }

            FilesystemType::UNKNOWN => {
                panic!("Trying to open \"{path}\" on slice with GUID {} failed: Unknown filesystem \nHalting.", slice.guid.as_string())
            }
        }
    }

    /// Reads 'count' bytes from the file into 'buffer'. Returns the amount of bytes that were read
    pub unsafe fn read_raw(&self, count: usize, buffer: *mut u8) -> usize {
        assert!(count <= self.filesize as usize);

        let fs_type = find_slice(self.slice).fs_type;
        match fs_type {
            FilesystemType::FAT => {
                // UEFI paths are Microsoft style '\' rather than '/'
                let path = String::from(format!("{}\0", self.path.to_string().to_ascii_uppercase().replace("/", "\\")));

                // First we need to access the filesystem protocol to open the root directory
                let filesys_protocol = BootServices::handle_protocol::<SimpleFilesystem>(find_slice(self.slice).handle);
                let file = filesys_protocol.open_volume();
              
                // Now we can open the file
                let file = file.open(path.as_str(), 1, None);

                let mut efi_count = count;
                unsafe { 
                    file.read(&mut efi_count, buffer);
                }
                

                efi_count

                // Close the EFI file
                // !!Needs to be done!!
            }

            FilesystemType::XFS => {
                //
                0
            }

            FilesystemType::UNKNOWN => {
                panic!("Error: Cannot read '{}' from slice with GUID {}: File not found \nHalting.", self.path, self.slice.as_string())
            }
        }
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

    pub fn get_position(&self) -> i64 {
        self.position
    }
}




/// Finds a slice by GUID
fn find_slice(guid: GUID) -> SliceInfo {
    unsafe {
        for slice in SLICE_ENTRIES.iter() {
            if slice.guid == guid {
                return *slice
            }
        }

        panic!("FS error: Could not find slice with GUID: {}. Halting.", guid.as_string());
    }
}




/// Returns the GUID partition signature of the ESP
pub fn get_esp_guid() -> GUID {
    unsafe {
        for slice in SLICE_ENTRIES.iter() {
            if slice.is_esp {
                return slice.guid
            }
        }
    }

    panic!("FS error: Could not find EFI System Partition. Halting.");
}




/// Detects the filesystem of the slice
fn detect_fs_type(guid: GUID) -> FilesystemType {
    FilesystemType::UNKNOWN
}




/// Initialize the filesystem driver
pub fn init() {

    // Get the EFI_HANDLE of the slice containing the EFI System Partition so we can label its FSInfo entry as such. It's important to note that multiple ESPs may be deteced, e.g if the user is booting from a memory stick
    // but also has a ESP on their primary hard disk. Matching the device handle of the boot slice to the partition signature of the slice is a reliable way to ensure we found the correct ESP, as opposed to searching for the ESP magic GUID
    let efi_sys_handle = BootServices::handle_protocol::<LoadedImageProtocol>(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst)).device_handle;


    // Get a list of handles that support the BlockIOProtocol. This list includes every storage media device + their partitions.
    let handles = BootServices::locate_handle_by_protocol::<BlockIOProtocol>();
    
    // Iterate through the handles, looking specifically for the Hard Drive Device Path node. The presence of this node indicates that the handle belongs to a slice
    let mut slice_info: Vec<SliceInfo> = Vec::new();

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
            
            
                    // See if we found the ESP or not
                    if handles[i] == efi_sys_handle as usize {
                        slice_info.push(SliceInfo::new(guid, handles[i] as *const usize, FilesystemType::FAT, true));
                        ldrprintln!("Found EFI System Partion with GUID: {}", guid.as_string());
                    }
                    else {
                        slice_info.push(SliceInfo::new(guid, handles[i] as *const usize, detect_fs_type(guid), false));
                        ldrprintln!("Found slice with GUID: {}", guid.as_string());
                    }
                }

                _ => {}
            }

            node = node.next();
        }
    }

    assert!(slice_info.is_empty() == false);
    unsafe { SLICE_ENTRIES = slice_info; }
}