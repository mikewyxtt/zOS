/*  extfs.rs - EXT filesystem driver
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

use alloc::{boxed::Box, vec, vec::Vec};
use crate::uuid::GUID;
use crate::firmware::{self, disk};
use core::mem::size_of;
use core::ptr;


#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Ext4Superblock {
    pub inodes_count:               u32,
    pub blocks_count_lo:            u32,
    pub r_blocks_count_lo:          u32,
    pub free_blocks_count_lo:       u32,
    pub free_inodes_count:          u32,
    pub first_data_block:           u32,
    pub log_block_size:             u32,
    pub log_cluster_size:           u32,
    pub blocks_per_group:           u32,
    pub clusters_per_group:         u32,
    pub inodes_per_group:           u32,
    pub mtime:                      u32,
    pub wtime:                      u32,
    pub mnt_count:                  u16,
    pub max_mnt_count:              u16,
    pub magic:                      u16,
    pub state:                      u16,
    pub errors:                     u16,
    pub minor_rev_level:            u16,
    pub lastcheck:                  u32,
    pub checkinterval:              u32,
    pub creator_os:                 u32,
    pub rev_level:                  u32,
    pub def_resuid:                 u16,
    pub def_resgid:                 u16,
    pub first_ino:                  u32,
    pub inode_size:                 u16,
    pub block_group_nr:             u16,
    pub feature_compat:             u32,
    pub feature_incompat:           u32,
    pub feature_ro_compat:          u32,
    pub uuid:                       [u8; 16],
    pub volume_name:                [u8; 16],
    pub last_mounted:               [u8; 64],
    pub algorithm_usage_bitmap:     u32,
    pub prealloc_blocks:            u8,
    pub prealloc_dir_blocks:        u8,
    pub reserved_gdt_blocks:        u16,
    pub journal_uuid:               [u8; 16],
    pub journal_inum:               u32,
    pub journal_dev:                u32,
    pub last_orphan:                u32,
    pub hash_seed:                  [u32; 4],
    pub def_hash_version:           u8,
    pub jnl_backup_type:            u8,
    pub desc_size:                  u16,
    pub default_mount_opts:         u32,
    pub first_meta_bg:              u32,
    pub mkfs_time:                  u32,
    pub jnl_blocks:                 [u32; 17],
    pub blocks_count_hi:            u32,
    pub r_blocks_count_hi:          u32,
    pub free_blocks_count:          u32,
    pub min_extra_isize:            u16,
    pub want_extra_isize:           u16,
    pub flags:                      u32,
    pub raid_stride:                u16,
    pub mmp_interval:               u16,
    pub mmp_block:                  u64,
    pub raid_stripe_width:          u32,
    pub log_groups_per_flex:        u8,
    pub checksum_type:              u8,
    pub reserved_pad:               u16,
    pub kbytes_written:             u64,
    pub snapshot_inum:              u32,
    pub snapshot_id:                u32,
    pub snapshot_r_blocks_count:    u64,
    pub snapshot_list:              u32,
    pub error_count:                u32,
    pub first_error_time:           u32,
    pub first_error_ino:            u32,
    pub first_error_block:          u64,
    pub first_error_func:           [u8; 32],
    pub first_error_line:           u32,
    pub last_error_time:            u32,
    pub last_error_ino:             u32,
    pub last_error_line:            u32,
    pub last_error_block:           u64,
    pub last_error_func:            [u8; 32],
    pub mount_opts:                 [u8; 64],
    pub usr_quota_inum:             u32,
    pub grp_quota_inum:             u32,
    pub overhead_blocks:            u32,
    pub backup_bgs:                 [u32; 2],
    pub encrypt_algos:              [u8; 4],
    pub encrypt_pw_salt:            [u8; 16],
    pub lpf_ino:                    u32,
    pub prj_quota_inum:             u32,
    pub checksum_seed:              u32,
    pub wtime_hi:                   u8,
    pub mtime_hi:                   u8,
    pub mkfs_time_hi:               u8,
    pub lastcheck_hi:               u8,
    pub first_error_time_hi:        u8,
    pub last_error_time_hi:         u8,
    pub pad:                        [u8; 2],
    reserved:                       [u32; 96],
    pub checksum:                   u32,
}


#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Ext4BlockGroupDescriptor {
    pub block_bitmap_lo:            u32,
    pub inode_bitmap_lo:            u32,
    pub inode_table_lo:             u32,
    pub free_blocks_count_lo:       u16,
    pub free_inodes_count_lo:       u16,
    pub used_dirs_count_lo:         u16,
    pub flags:                      u16,
    pub exlcude_bitmap_lo:          u32,
    pub block_bitmap_csum_lo:       u16,
    pub inode_bitmap_csum_lo:       u16,
    pub itable_unused_lo:           u16,
    pub checksum:                   u16,
    pub block_bitmap_hi:            u32,
    pub inode_bitmap_hi:            u32,
    pub inode_table_hi:             u32,
    pub free_blocks_count_hi:       u16,
    pub free_inodes_count_hi:       u16,
    pub used_dirs_count_hi:         u16,
    pub itable_unused_hi:           u16,
    pub exclude_bitmap_hi:          u32,
    pub block_bitmap_csum_hi:       u16,
    pub inode_bitmap_csum_hi:       u16,
    reserved:                       u32,
}



#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Ext4INode {
    pub mode:                       u16,
    pub uid:                        u16,
    pub size_lo:                    u32,
    pub atime:                      u32,
    pub ctime:                      u32,
    pub mtime:                      u32,
    pub dtime:                      u32,
    pub gid:                        u16,
    pub links_count:                u16,
    pub blocks_lo:                  u32,
    pub flags:                      u32,
    pub osd1:                       [u8; 4],
    pub block:                      [u8; 60],
    pub generation:                 u32,
    pub file_acl_lo:                u32,
    pub size_high:                  u32,
    obso_faddr:                     u32,
    pub osd2:                       [u8; 12],
    pub extra_isize:                u16,
    pub checksum_hi:                u16,
    pub ctime_extra:                u32,
    pub mtime_extra:                u32,
    pub atime_extra:                u32,
    pub crtime:                     u32,
    pub crtime_extra:               u32,
    pub version_hi:                 u32,
    pub projid:                     u32,
}

#[repr(C, packed)]
struct ExtentHeader {
    pub magic:      u16,
    pub entries:    u16,
    pub max:        u16,
    pub depth:      u16,
    pub generation: u32,
}

#[repr(C, packed)]
struct ExtentIndex {
    pub block:      u32,
    pub leaf_lo:    u32,
    pub leaf_hi:    u16,
    unused:         u16,
}

#[repr(C, packed)]
struct ExtentLeaf {
    pub block:      u32,
    pub len:        u16,
    pub start_hi:   u16,
    pub start_lo:   u32,
}



#[repr(C, packed)]
struct Ext4DirectoryEntry {
    pub inode:                      u32,
    pub rec_len:                    u16,
    pub name_len:                   u8,
    pub file_type:                  u8,
    pub name:                       [u8],
}

impl Ext4Superblock{
    pub fn new_zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}

impl Ext4BlockGroupDescriptor {
    pub fn new_zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}


impl Ext4INode {
    pub fn new_zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }

    pub const fn get_block_group(sb: &Ext4Superblock, inode_num: u32) -> u32 {
        (inode_num - 1) / sb.inodes_per_group
    }

    pub const fn get_index(sb: &Ext4Superblock, inode_num: u32) -> u32 {
        (inode_num - 1) % sb.inodes_per_group
    }

    pub const fn get_offset(sb: &Ext4Superblock, inode_num: u32) -> u32 {
        let index = Self::get_index(sb, inode_num);

        index * sb.inode_size as u32
    }
}




/// Scans the slice to determine if it contains an Ext filesystem. Returns true if it is.
pub fn detect(slice: GUID) -> bool {
    let sb = {
        let mut buff: Box<Ext4Superblock> = Box::new(Ext4Superblock::new_zeroed());
        unsafe { 
            disk::read_bytes_raw(slice, 2, size_of::<Ext4Superblock>(), (buff.as_mut() as *mut Ext4Superblock).cast()).unwrap();
        }

        buff
    };

    if u16::from_le(sb.magic) == 0xEF53 {
        ldrprintln!("Found Ext4 filesystem on slice with GUID '{}'", slice.as_string());
        return true
    }
    else {
        return false
    }
}



/// Reads an inode from the disk
fn read_inode(slice: GUID, inode_num: u32) -> Result<Box<Ext4INode>, ()> {
    // bring superblock into memory
    let sb = {
        let mut buff: Box<Ext4Superblock> = Box::new(Ext4Superblock::new_zeroed());
        unsafe { 
            disk::read_bytes_raw(slice, 2, size_of::<Ext4Superblock>(), (buff.as_mut() as *mut Ext4Superblock).cast()).unwrap();
        }

        buff
    };

    // Read the block group descriptors into memory
    let bg_descriptors: Vec<Ext4BlockGroupDescriptor> = {
        // TODO: Need to make this dynamic, not a static LBA and capacity..
        let mut buff: Vec<Ext4BlockGroupDescriptor> = vec![Ext4BlockGroupDescriptor::new_zeroed(); 10];
        unsafe {
            disk::read_bytes_raw(slice, 8,buff.len() * size_of::<Ext4BlockGroupDescriptor>(), buff.as_mut_ptr() as *mut u8).unwrap();
        }

        buff
    };

    // Get the block group descriptor for the inode
    let bg_descriptor = { 
        let block_group_num: usize = Ext4INode::get_block_group(&sb, inode_num).try_into().unwrap();

        bg_descriptors[block_group_num]
    };

    let table_loc = u32::from_le(bg_descriptor.inode_table_lo);
    let phys_blk_size: u64 = disk::get_phys_block_size(slice).try_into().unwrap();
    let log_b_size: u32 = 1 << (10 + u32::from_le(sb.log_block_size));    
    let inode_table_lba: u64 = table_loc as u64 * (log_b_size as u64 / phys_blk_size);
    let inode_size = u16::from_le(sb.inode_size);

    // Index of our inode within its inode table
    let inode_index: usize = Ext4INode::get_index(&sb, inode_num).try_into().unwrap();

    // Location of the inode as an offset in bytes starting from the inode table LBA
    let inode_phys_offset = inode_index as usize * inode_size as usize;

    // how many sectors we need to offset by within the inode table
    let inode_offset_lba = inode_phys_offset as usize / phys_blk_size as usize;
    let inode_lba = inode_table_lba as usize + inode_offset_lba as usize;

    // Read the sector into a buffer
    let mut buffer: Vec<u8> = vec![0; phys_blk_size as usize];
    firmware::disk::read_bytes(slice, inode_lba as u64, phys_blk_size as usize, &mut buffer).unwrap();

    // Pick the inode out of the buffer
    let mut inode: Box<Ext4INode> = Box::new(Ext4INode::new_zeroed());
    let offset: usize = inode_phys_offset - (inode_offset_lba * phys_blk_size as usize);
    unsafe {
        core::ptr::copy(&buffer[offset], (inode.as_mut() as *mut Ext4INode).cast(), size_of::<Ext4INode>());
    }

    // TODO: Do some sanity checks
    Ok(inode)
}




/// Searches the directory entries for a file, and if found returns its inode number
fn get_file_inode(slice: GUID, path: &str) -> Result<u32, &str> {
    // Start at the root directory inode (always inode 2)
    let mut inode_num = 2;
    let path = path.trim_start_matches("/");
    for path_entry in path.split("/") {
        let inode = read_inode(slice, inode_num).unwrap();
        if u16::from_le_bytes([inode.block[0], inode.block[1]]) == 0xF30A {

            // parse the extent tree OR block map (check for the flag)
            let _ext_tree_header: &ExtentHeader = unsafe { &*ptr::from_raw_parts((&inode.block[0] as *const u8).cast(), ()) };
            let ext_tree_leaf: &ExtentLeaf = unsafe { &*ptr::from_raw_parts((&inode.block[12] as *const u8).cast(), ()) };

            // read its contents into memory (the directory entries)
            let buff_size = u32::from_le(inode.size_lo) as usize;
            let lba = (ext_tree_leaf.start_lo as u64 * 4096) / firmware::disk::get_phys_block_size(slice);
            let mut buff: Vec<u8> = vec![0; buff_size];
            firmware::disk::read_bytes(slice, lba, buff_size, &mut buff).unwrap();

            // Parse the entries
            let mut i = 0;
            while i < buff_size {
                // Get the static part of the dir entry
                let dir_entry: &Ext4DirectoryEntry = unsafe { &*ptr::from_raw_parts((&buff[i] as *const u8).cast(), 0) };
                let dir_entry: &Ext4DirectoryEntry = unsafe { &*ptr::from_raw_parts((&buff[i] as *const u8).cast(), u8::from_le(dir_entry.name_len) as usize) };

                // let dir_entry: &Ext4DirectoryEntry = unsafe { ptr::from_raw_parts::<Ext4DirectoryEntry>((&buff[i] as *const u8).cast(), 0).as_ref().unwrap() };

                if path_entry == core::str::from_utf8(&dir_entry.name).unwrap() {
                    // Regular file?>
                    if dir_entry.file_type == 0x1 {
                        return Ok(u32::from_le(dir_entry.inode));

                    }
                    else {
                        inode_num = u32::from_le(dir_entry.inode);
                    }
                }

                i += u16::from_le(dir_entry.rec_len) as usize;
            }
        }
        else {
            panic!("Data block parsing for EXT filesystems is not implemented. FS must use an extent tree.")
        }
    }

    Err("Could not find file")

}




/// Reads a files entire contents into *buffer*
///
/// If *buffer* is a null ptr, this fn returns the buffer size needed to contain the file. Otherwise, it returns None.
pub unsafe fn read_bytes_raw(slice: GUID, path: &str, buffer: *mut u8) -> Option<u64> {
    let inode = read_inode(slice, get_file_inode(slice, path).unwrap()).unwrap();

    if buffer.is_null() {
        ldrprintln!("null ptr, returning filesize: {}", u32::from_le(inode.size_lo) as u64);
        return Some(u32::from_le(inode.size_lo) as u64);
    }
    else {
        // Parse the extent tree OR block map (check for the flag)
        // Magic number must be 0xF30A (always little endian) if the fs is using an extent tree instead of a block map
        if u16::from_le_bytes([inode.block[0], inode.block[1]]) == 0xF30A {
            let _ext_tree_header: &ExtentHeader = unsafe { &*ptr::from_raw_parts((&inode.block[0] as *const u8).cast(), ()) };
            let ext_tree_leaf: &ExtentLeaf = unsafe { &*ptr::from_raw_parts((&inode.block[12] as *const u8).cast(), ()) };

            // read the files contents into the buffer
            let buff_size = u32::from_le(inode.size_lo) as usize;
            let lba = (ext_tree_leaf.start_lo as u64 * 4096) / firmware::disk::get_phys_block_size(slice);
            unsafe { 
                firmware::disk::read_bytes_raw(slice, lba, buff_size, buffer).unwrap(); 
            }
            return None;
        }
        else {
            panic!("Data block parsing for EXT filesystems is not implemented. FS must use an extent tree.");
        }
    }
}