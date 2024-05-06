/*  fat.rs - Basic FAT fs driver
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

use core::mem::size_of;
use alloc::{boxed::Box, vec::Vec, vec};
use crate::{libuefi::GUID, uefi::disk};
use super::fs::File;

#[derive(PartialEq, Eq)]
enum FATType {
    FAT12,
    FAT16,
    FAT32,
}

#[repr(C, packed)]
struct BiosParameterBlock {
    jmp_boot:               [u8; 3],
    pub oem_name:           [u8; 8],
    pub bytspersec:         u16,
    pub secperclus:         u8,
    pub rsvdseccnt:         u16,
    pub numfats:            u8,
    pub rootentcnt:         u16,
    pub totsec16:           u16,
    pub media:              u8,
    pub fatsz16:            u16,
    pub secpertrack:        u16,
    pub numheads:           u16,
    pub hiddsec:            u32,
    pub totsec32:           u32,
    pub fatzs32:            u32,

    pub extflags:           u16,
    pub fsver:              u16,
    pub rootclus:           u32,
    pub fsinfo:             u16,
    pub bkbootsec:          u16,
    _reserved:              [u8; 12],
}

#[repr(C, packed)]
struct FSInfo {
    pub leadsig:            u32,
    _reserved1:             [u8; 480],
    pub strucsig:           u32,
    pub free_count:         u32,
    pub nxt_free:           u32,
    _reserved2:             [u8; 12],
    pub trailsig:           u32,
}

#[repr(C, packed)]
struct DirectoryEntry {
    pub name:               [u8; 11],
    pub attr:               u8,
    nt_rsvd:                u8,
    pub crt_time_tenth:     u8,
    pub crt_time:           u16,
    pub crt_date:           u16,
    pub lst_acc_date:       u16,
    pub fst_clus_hi:        u16,
    pub wrt_time:           u16,
    pub wrt_date:           u16,
    pub fst_clus_lo:        u16,
    pub filesize:           u32,
}

impl BiosParameterBlock {
    pub fn zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}



/// Detects whether or not the slice contains a FAT filesystem of any kind.
///
/// It should be noted that these strings are not used to determine the fat type, as Microsoft states you must determine FAT type using the total count of clusters and nothing else.
pub fn detect(slice: GUID) -> bool {

    // Read the boot sector into memory
    let bs = {
        let mut buffer: Vec<u8> = vec![0; 512];
        let _ = unsafe { disk::read_bytes_raw(slice, 0, 512, buffer.as_mut_ptr()) };

        buffer
    };


    // There are 'filesystype' fields in the FAT 12/16 and FAT32 BPB blocks with a string that reads one of: "FAT     ", "FAT12   ", "FAT16   ", or "FAT32   "
    // We only check for "FAT" for maximum compatibility, as some tools may only put "FAT" in this field. The actual FAT type will be determined later on using the official algorithm.
    if &bs[54..57] == b"FAT" ||  &bs[82..85] == b"FAT" {
        return true;
    }
    else {
        return false;
    }
}



fn detect_fat_type(bpb: &BiosParameterBlock) -> FATType {
    let root_dir_sectors = ((bpb.rootentcnt * 32) + (bpb.bytspersec - 1)) / bpb.bytspersec;

    let fat_size: u32;
    let total_sectors;

    if bpb.fatsz16 != 0 {
        fat_size = bpb.fatsz16.into();
    }
    else {
        fat_size = bpb.fatzs32;
    }

    if bpb.totsec16 != 0 {
        total_sectors = bpb.totsec16.into();
    }
    else {
        total_sectors = bpb.totsec32;
    }

    let data_sectors = total_sectors - (bpb.rsvdseccnt as u32 + (bpb.numfats as u32 * fat_size as u32) + root_dir_sectors as u32);

    let count_of_clusters = data_sectors / bpb.secperclus as u32;

    if count_of_clusters < 4085 {
        return FATType::FAT12;
    }
    else if count_of_clusters < 65525 {
        return FATType::FAT16;
    }
    else {
        return FATType::FAT32;
    }
}




const fn find_first_sector_of_cluster(bpb: &BiosParameterBlock, cluster_number: u32) -> u32 {
    let root_dir_sectors = ((bpb.rootentcnt * 32) + (bpb.bytspersec - 1)) / bpb.bytspersec;

    let fat_size: u32;

    if bpb.fatsz16 != 0 {
        fat_size = bpb.fatsz16 as u32;
    }
    else {
        fat_size = bpb.fatzs32 as u32;
    }

    let first_data_sector: u32 = bpb.rsvdseccnt as u32 + (bpb.numfats as u32 * fat_size) + root_dir_sectors as u32;


    ((cluster_number - 2) * bpb.secperclus as u32) + first_data_sector
}




/// Determines whether the cluster contains the EOF mark or not
const fn is_eof(fat_type: FATType, fat_content: u32) -> bool {
    match fat_type {
        FATType::FAT12 => {
            if fat_content >= 0x0FF8 {
                return true;
            }
            else {
                return false;
            }
        }

        FATType::FAT16 => {
            if fat_content >= 0xFFF8 {
                return true;
            }
            else {
                return false;
            }
        }

        FATType::FAT32 => {
            if fat_content >= 0x0FFFFFF8 {
                return true;
            }
            else {
                return false;
            }
        }
    }
}




fn conv_to_8_3_name(name: &str) -> [u8; 11] {
    assert!(name.len() <= 11, "Long filenames are not supported by the zOS FAT driver.");

    // Create an empty filename, filled with spaces
    let mut filename8_3 = [0x20u8; 11];

    // If its a file name
    if name.contains('.') && name.len() > 1 {
        let (filename, ext) = name.split_once('.').unwrap();

        for (i, b) in filename.as_bytes().iter().enumerate() {
            filename8_3[i] = *b;
        }

        for (i, b) in ext.as_bytes().iter().enumerate() {
            filename8_3[8 + i] = *b;
        }

        return filename8_3;

    }

    else {
        for (i, b) in name.as_bytes().iter().enumerate() {
            filename8_3[i] = *b;
        }

        return filename8_3;
    }
}





fn find_file(slice: GUID, path: &str, bpb: &BiosParameterBlock) -> Result<DirectoryEntry, ()> {
    let path = path.to_uppercase();
    let path = path.trim_start_matches("/");

    // Start at the root directory
    let mut cluster_num = bpb.rootclus;

    // Iterate thru each part of the path
    for path_entry in path.split("/") {
        let path_entry = conv_to_8_3_name(path_entry);

        // Load the directories
        let dir_entries = {
            let num_dir_entries = (bpb.bytspersec as usize * bpb.secperclus as usize) / 32;

            let mut entries: Vec<DirectoryEntry> = Vec::with_capacity(num_dir_entries);
            unsafe { entries.set_len(num_dir_entries); }
            let lba = find_first_sector_of_cluster(&bpb, cluster_num);
            let _ = unsafe { disk::read_bytes_raw(slice, lba as u64, num_dir_entries * size_of::<DirectoryEntry>(), entries.as_mut_ptr().cast()) };

            entries
        };

        for entry in dir_entries {
            // Last entry?
            if entry.name[0] == 0xE5 || entry.name[0] == 0x00 {
                break;
            }
            else if entry.name == path_entry {
                // Found the file!
                if entry.attr != 0x10 {
                    return Ok(entry);
                }
                // We found the dir, continue to the next directory entry
                else {
                    cluster_num = entry.fst_clus_lo as u32;
                    break;
                }
            }
        }
    }

    return Err(())
}




unsafe fn read_raw(file: &File, count: usize, buffer: *mut u8) {
    let bpb = {
        let mut buffer: Box<BiosParameterBlock> = Box::new(BiosParameterBlock::zeroed());
        let _ = unsafe { disk::read_bytes_raw(file.slice, 0, size_of::<BiosParameterBlock>(), (buffer.as_mut() as *mut BiosParameterBlock).cast()) };

        buffer
    };

    // NOTE: For the sake of simplicity, i skipped implementing the ability to parse thru the FAT entires to read a big file, as this is literally only used for loader.cfg
    let cluster_size = bpb.secperclus as u16 * bpb.bytspersec as u16;
    assert!(count < cluster_size.into(), "Reading files larger than one FAT cluster({} bytes) from FAT slices is not supported by the zOS FAT driver.", cluster_size);

    
    let entry = find_file(file.slice, &file.path, bpb.as_ref()).unwrap();
    let lba = find_first_sector_of_cluster(&bpb, entry.fst_clus_lo.into());
    let _ = unsafe { disk::read_bytes_raw(file.slice, lba.into(), count, buffer) };
}




pub fn open(slice: GUID, path: &str) -> File {
    let bpb = {
        let mut buffer: Box<BiosParameterBlock> = Box::new(BiosParameterBlock::zeroed());
        let _ = unsafe { disk::read_bytes_raw(slice, 0, size_of::<BiosParameterBlock>(), (buffer.as_mut() as *mut BiosParameterBlock).cast()) };

        buffer
    };

    match detect_fat_type(&bpb) {
        FATType::FAT32 => {
            

            let f = find_file(slice, path, bpb.as_ref()).expect("File not found.");

            let filesize = f.filesize;
            return File::new(slice, path, filesize.into(), read_raw);
            
        }

        _ => {
            panic!();
        }
    }
}