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
use crate::{drivers::uefi::disk, libuefi::{bootservices::BootServices, protocol::{block_io::BlockIOProtocol, device_path::{DevicePathProtocol, HardDriveDevicePath}, file::FileInfo, filesystem::SimpleFilesystem, loaded_image::LoadedImageProtocol}, GUID}};


static mut SLICE_ENTRIES: Vec<FSInfo> = Vec::new();


#[derive(Clone, Copy, PartialEq, Eq)]
enum FilesystemType {
    FAT,
    XFS,
    Unknown,
}



#[derive(Clone, Copy)]
struct FSInfo {
    guid:       GUID,
    handle:     *const usize,
    fs_type:    FilesystemType,
    is_esp:     bool,
}

impl FSInfo {
    pub const fn new(guid: GUID, handle: *const usize, fs_type: FilesystemType, is_esp: bool) -> Self {
        Self {
            guid,
            handle,
            fs_type,
            is_esp
        }
    }
}


pub struct File {
    filesystem_type:    FilesystemType,
    contents: Vec<u8>,
    position: u64,
    filesize: u64,
}


impl File {
    pub fn open(slice: GUID, path: &str) -> Self {

        let slice = find_slice(slice);


        match slice.fs_type {
            FilesystemType::FAT => {
                // // UEFI paths are Microsoft style '\' rather than '/'
                let path = String::from(format!("{}\0", path.to_string().to_ascii_uppercase().replace("/", "\\")));


                // First we need to access the filesystem protocol
                let filesys_protocol = BootServices::handle_protocol::<SimpleFilesystem>(slice.handle);

                // Then we open the volume and open the file
                let file = filesys_protocol.open_volume();
                let file = file.open(path.as_str(), 1, None);
                let info = file.get_info(FileInfo::guid());

                // Read the files contents into a buffer
                let mut count: usize = info.file_size as usize;
                let mut contents: Vec<u8> = Vec::with_capacity(count);
                file.read(&mut count, &mut contents);

                // Close the EFI file
                // Needs to be done

                return Self {
                    filesystem_type: FilesystemType::FAT,
                    contents,
                    position: 0,
                    filesize: count as u64,
                }
            }

            FilesystemType::XFS => {
                //
            }

            FilesystemType::Unknown => {
                //
            }
        }

        Self {
            filesystem_type: slice.fs_type,
            contents: Vec::new(),
            filesize: 0,
            position: 0
        }
    }

    
    pub fn readln(&mut self) -> (String, bool) {
        let mut s = String::new();

        while self.contents[self.position as usize] != b'\n' {
            if self.position >= self.filesize {
                self.position = 0;
                break;
            }
            s.push(self.contents[self.position as usize] as char);
            self.position += 1;

        }
        self.position +=1;

        if self.position >= self.filesize {
            self.position = 0;
            return (s, true)
        }

        (s, false)
    }
}



/// Finds a slice by GUID
fn find_slice(guid: GUID) -> FSInfo {
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
    FilesystemType::Unknown
}




/// Initialize the filesystem driver
pub fn init() {

    // Get the EFI_HANDLE of the slice containing the EFI System Partition so we can label its SliceEntry as such
    let efi_sys_handle = BootServices::handle_protocol::<LoadedImageProtocol>(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst)).device_handle;


    // Get a list of handles that support the BlockIOProtocol. This list includes every storage media device + their partitions.
    let handles = BootServices::locate_handle_by_protocol::<BlockIOProtocol>();
    
    // Iterate through the handles, looking specifically for the Hard Drive Device Path node. This specific node indicates that the handle belongs to a slice
    let mut fs_info: Vec<FSInfo> = Vec::new();

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
                        fs_info.push(FSInfo::new(guid, handles[i] as *const usize, FilesystemType::FAT, true));
                        ldrprintln!("Found EFI System Partion with GUID: {}", guid.as_string());
                    }
                    else {
                        fs_info.push(FSInfo::new(guid, handles[i] as *const usize, detect_fs_type(guid), false));
                        ldrprintln!("Found slice with GUID: {}", guid.as_string());
                    }
                }

                _ => {}
            }

            node = node.next();
        }
    }

    assert!(fs_info.is_empty() == false);
    unsafe { SLICE_ENTRIES = fs_info; }
}