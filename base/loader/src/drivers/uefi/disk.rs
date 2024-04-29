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
use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::libuefi::{bootservices::BootServices, protocol::{block_io::BlockIOProtocol, device_path::{DevicePathProtocol, HardDriveDevicePath}, file::FileInfo, filesystem::SimpleFilesystem, loaded_image::LoadedImageProtocol}, GUID};


static mut SLICE_ENTRIES: Vec<SliceEntry> = Vec::new();

#[derive(Clone, Copy)]
struct SliceEntry {
    guid:                   GUID,
    handle:                 *const usize,
    pub fs_type:            FilesystemType,
    pub is_efi_sys:         bool,
}

impl SliceEntry {
    pub const fn new(guid: GUID, handle: *const usize, fs_type: FilesystemType, is_efi_sys: bool) -> Self {
        Self {
            guid,
            handle,
            fs_type,
            is_efi_sys,
        }
    }
}


/// Fills SLICE_ENTRIES with a list of slice GUIDs and their EFI Handle
pub fn probe_disks() {

    // Get the EFI_HANDLE of the slice containing the EFI System Partition so we can label its SliceEntry as such
    let efi_sys_handle = BootServices::handle_protocol::<LoadedImageProtocol>(crate::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst)).device_handle;


    // Get a list of handles that support the BlockIOProtocol. This list includes every storage media device + their partitions.
    let handles = BootServices::locate_handle_by_protocol::<BlockIOProtocol>();
    
    // Iterate through the handles, looking specifically for the Hard Drive Device Path node. This specific node indicates that the handle belongs to a slice
    let mut partition_entries: Vec<SliceEntry> = Vec::new();

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
                        partition_entries.push(SliceEntry::new(guid, handles[i] as *const usize, FilesystemType::FAT, true));
                        ldrprintln!("Found EFI SYSTEM PARTITION with GUID: {}", guid.as_string());
                    }
                    else {
                        partition_entries.push(SliceEntry::new(guid, handles[i] as *const usize, FilesystemType::Unknown, false));
                        ldrprintln!("Found partition with GUID: {}", guid.as_string());
                    }
                }

                _ => {}
            }

            node = node.next();
        }
    }

    assert!(partition_entries.is_empty() == false);
    unsafe { SLICE_ENTRIES = partition_entries; }
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
        assert_eq!(SLICE_ENTRIES.is_empty(), false);
        
        for partition in SLICE_ENTRIES.iter() {
            if partition.guid.as_string() == guid.as_string() {
                return partition.handle
            }
        }
    }

    panic!("Partition not found: {}", guid.as_string());
}



/// Finds a SliceEntry by GUID
fn find_slice(guid: GUID) -> SliceEntry {
    unsafe {
        assert_eq!(SLICE_ENTRIES.is_empty(), false);
        
        for partition in SLICE_ENTRIES.iter() {
            if partition.guid == guid {
                return *partition
            }
        }

        panic!("Could not find slice with GUID: {}.", guid.as_string());
    }
}



/// Finds the EFI System Partition's slice entry
fn find_esp_slice() -> SliceEntry {
    unsafe {
        assert_eq!(SLICE_ENTRIES.is_empty(), false);
        
        for partition in SLICE_ENTRIES.iter() {
            if partition.is_efi_sys == true {
                return *partition
            }
        }

        panic!("Could not find EFI System Partition.");
    }
}




/// Parses a key="value" pair
pub fn parse_key_value_pair(line: &str) -> (String, String) {

    if !line.contains('=') {
        ldrprintln!("WARNING: Unknown configuration line \"{}\". Ignoring.", line);
        return (String::new(), String::new())
    }

    let (key, value) = line.split_once('=').unwrap();
    let key = key.trim();
    let value = value.trim();
    let value = value.trim_matches('"');

    (key.to_string(), value.to_string())
}



pub struct Config {
    pub rootfs:         GUID,
    pub resolution:     String,
}




impl Default for Config {
    fn default() -> Self {
        Self {
            rootfs:     GUID::new(0,0,0, [0; 8]),
            resolution: String::from("native"),
        }
    }
}


/// Reads and parses the cfg file from the ESP
pub fn parse_cfg() -> Config {
    let mut config = Config::default();

    let mut f = File::open(find_esp_slice().guid, "/EFI/BOOT/ZOS/LOADER.CFG");

    let mut eof = false;
    let mut line: String;

    while !eof {
        (line, eof) = f.readln();
        let (key, value) = parse_key_value_pair(line.as_str());

        match key.as_str() {
            "root" => {
                config.rootfs = GUID::new_from_string(&value);
            }

            "resolution" => {
                config.resolution=value;
            }

            _=> { ldrprintln!("WARNING: Unknown configuration option \"{}\". Ignoring.", key); }
        }
    }

    config
}

struct File {
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
                // UEFI paths are Microsoft style '\' rather than '/'
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum FilesystemType {
    FAT,
    XFS,
    Unknown,
}