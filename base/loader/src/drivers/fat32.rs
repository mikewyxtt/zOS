/*  fat32.rs - FAT driver
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

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use debugutils::hexdump;

use super::disk::*;

#[repr(C, packed)]
struct BIOSParameterBlock {
    _jmp:                           [u8; 3],    // BIOS starts reading from here, jmp instruction jumps over this struct
    pub oem_name:                   [u8; 8],
    pub bytes_per_sector:           u16,
    pub sectors_per_cluster:        u8,
    pub reserved_sector_cnt:        u16,
    pub num_fats:                   u8,
    pub root_entry_count:           u16,
    pub total_sectors_legacy:       u16,        // FAT12/FAT16 only
    pub media_id:                   u8,
    pub sectors_per_fat_legacy:     u16,        // FAT12/FAT16 only
    pub sectors_per_track:          u16,        // unused, we use LBA
    pub num_heads:                  u16,        // unused, we use LBA
    pub hidden_sectors:             u16,        // unused, it represents hiddden sectors prior to this partition. N/A for a slice
    pub total_sectors_new:          u32,        // New total sector count for FAT32
    pub drive_num:                  u32,        // unused BIOS drive number of the disk
    _rsrvd1:                        u8,
    pub boot_signature:             u8,
    pub volume_id:                  u32,        // unused, useful for tracking removable media or something
    pub volume_label:               [u8; 11],   // ^^
    pub filesystem_type:            [u8; 8],    // unused, FAT type is determined via algorithm not a string
    pub sectors_per_fat_new:        u32,        // New sectors per FAT for FAT32
    pub ext_flags:                  u16,        // something to do with FAT mirroring
    pub fs_version:                 u16,        // filesystem version
    pub root_cluster:               u32,        // FAT32 only. Root cluster number
    pub fs_info:                    u16,        // Sector number of FS Info struct
    pub backup_boot_sec:            u16,        // FAT32 only. sector number of backup boot sector
    _reserved1:                     [u8; 12],   // FAT32 only
    pub drive_number:               u8,         // unused, BIOS drive number
    _reserved2:                     u8,
    pub boot_sig2:                  u8,         // Boot signature
    pub volume_id1:                 u32,        
    pub volume_label1:              [u8; 11],
    pub fs_type:                    [u8; 8],
}


// 
impl BIOSParameterBlock {
    pub fn new() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}

#[repr(C, packed)]
struct FSInfo {
    pub lead_sig:           u32,
    _reserved:              [u8; 480],
    pub struc_sig:          u32,
    pub free_cluster_count: u32,
    pub next_free:          u32,
    _reserved2:             [u8; 12],
    pub trail_sig:          u32,
}

#[repr(C, packed)]
struct DirectoryEntry {
    pub name:               [u8; 11],
    pub attributes:         u8,
    _rsrvd:                 u8,
    pub time_tenth:         u8,
    pub time_created:       u16,
    pub date_created:       u16,
    pub last_access_date:   u16,
    pub clust_high:         u16,
    pub write_time:         u16,
    pub write_date:         u16,
    pub clust_low:          u16,
    pub file_size:          u32,
}





#[repr(C, packed)]
struct LongDirectoryEntry {
    pub dir_order:          u8,
    pub name1:              [u8; 10],
    pub attributes:         u8,
    pub _type:              u8,
    pub checksum:           u8,
    pub name2:              [u8; 12],
    _fat_cluster_low:       u16,
    pub name3:              [u8; 4],
}





pub fn open(device: &str, path: &str) -> Result<Vec<u8>, ()> {
    // find the file
    // get the size
    // create a buffer
    let file: Vec<u8> = vec![0; 100];
    Ok(file)
}




fn find_file() {
    // find the file
    let mut bpb = BIOSParameterBlock::new();

    unsafe { core::ptr::copy(read_bytes("disk0s1", 0, core::mem::size_of::<BIOSParameterBlock>()).unwrap().as_mut_ptr(), (&mut bpb as *mut BIOSParameterBlock).cast(), 1 ); }
}




#[derive(PartialEq)]
enum TypeOfFAT {
    FAT12,
    FAT16,
    FAT32,
}
impl TypeOfFAT {
    const fn check(bpb: &BIOSParameterBlock) -> TypeOfFAT {
        let spf: u32;
        let total_sectors: u32;
        if bpb.sectors_per_fat_legacy != 0 && bpb.total_sectors_legacy != 0 {
            spf = bpb.sectors_per_fat_legacy as u32;
            total_sectors = bpb.total_sectors_legacy as u32;
        }
        else {
            spf = bpb.sectors_per_fat_new;
            total_sectors = bpb.total_sectors_new;
        }
    
        let root_dir_sectors = ((bpb.root_entry_count * 32) + (bpb.bytes_per_sector -1)) / bpb.bytes_per_sector;
        let data_sec = total_sectors - (bpb.reserved_sector_cnt as u32 + (bpb.num_fats as u32 * spf) + root_dir_sectors as u32) as u32;
        let cluster_count = data_sec / bpb.sectors_per_cluster as u32;
    
        if cluster_count < 4085 {
            TypeOfFAT::FAT12
        } 
        else if cluster_count < 65525 {
            TypeOfFAT::FAT16
        }
        else {
            TypeOfFAT::FAT32
        }
    }
}








pub fn read() {
    let bpb: Box<BIOSParameterBlock> = read_bytes_into_box("disk0s1", 0, core::mem::size_of::<BIOSParameterBlock>());

    let drive = bpb.sectors_per_fat_legacy;
    match TypeOfFAT::check(&bpb) {
        TypeOfFAT::FAT32 => {
            println!("FAT32 found");
            println!("drive num: {}", drive);
            println!("drive num: {:x}", drive);
        }

        _ => {}
    }

    // std::fs::File::open(path)
}

