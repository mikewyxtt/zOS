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


use alloc::boxed::Box;

use crate::{libuefi::GUID, uefi};
use core::mem::size_of;

#[repr(C, packed)]
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
struct Ext4DirectoryEntry {
    pub inode:                      u32,
    pub rec_len:                    u16,
    pub name_len:                   u8,
    pub file_type:                  u8,
    pub name:                       [u8],
}

// trait DiskDriver {
//     fn open();
//     fn close();
//     fn read_bytes_raw();
// }

// struct Driver;

// impl DriverTrait for Driver {
//     //
// }


/// Scans the slice to determine if it contains an Ext filesystem. Returns true if it is.
pub fn detect(slice: GUID) -> bool {
        let sb = {
        let mut buff: Box<Ext4Superblock> = unsafe { Box::new(core::mem::zeroed()) };
        let _ = unsafe { uefi::disk::read_bytes_raw(slice, 2, size_of::<Ext4Superblock>(), (buff.as_mut() as *mut Ext4Superblock).cast()) };

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

fn find_file() {
    //
}

