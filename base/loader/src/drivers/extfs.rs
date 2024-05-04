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
struct Superblock {
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
}

/// Scans the slice to determine if it contains an Ext filesystem. Returns true if it is.
pub fn detect(slice: GUID) -> bool {
    let mut sb: Box<Superblock> = unsafe { Box::new(core::mem::zeroed()) };
    let _ = unsafe { uefi::disk::read_bytes_raw(slice, 2, size_of::<Superblock>(), (sb.as_mut() as *mut Superblock).cast()) };

    if u16::from_le(sb.magic) == 0xEF53 {
        ldrprintln!("Found Ext4 filesystem on slice with GUID '{}'", slice.as_string());
        return true
    }
    else {
        return false
    }
}