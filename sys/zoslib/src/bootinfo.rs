/*  bootinfo.rs - BootInfo stuff
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

const MAX_MEMORY_MAP_ENTRIES: usize = 18;
const MAX_EXTENSION_COUNT: usize = 32;
const MAX_CMDLINE_SIZE: usize = 50;

const BOOTINFO_MAGIC: u16 = 0xFAFA;
const BOOTINFO_END: u16 = 0xFF77;

#[repr(C)]
pub struct MemoryMap {
    pub start:      usize,
    pub len:        usize,
    pub _type:      u8,
}


#[repr(C)]
pub struct Extension {
    pub name:       [char; 24],
    pub path:       [char; 100],
    pub addr:       usize,
    pub size:       usize,
}


#[repr(C)]
pub struct FBInfo {
    pub enabled:     bool,
    pub addr:        usize,
    pub width:       u32,
    pub height:      u32,
    pub pitch:       u32,
    pub depth:       u32,
    pub size:        u64,
}


#[repr(C)]
pub struct BootInfo {
    pub magic:          u16,
    pub version:        [char; 8],
    pub size:           usize,

    pub cmdline:        [char; MAX_CMDLINE_SIZE],               // Boot command line
    pub fb_info:        FBInfo,
    pub memory_map:     [MemoryMap; MAX_MEMORY_MAP_ENTRIES],
    pub extensions:     [Extension; MAX_EXTENSION_COUNT],
    pub end:            u16,
}



impl BootInfo {
    /// Returns an empty BootInfo struct
    pub fn new_empty() -> Self {
        let mut bootinfo = unsafe { core::mem::zeroed::<Self>() };
        bootinfo.size = size_of::<Self>();

        bootinfo
    }

    // Returns a reference to the BootInfo struct. Performs checks on magic numbers and size to ensure the reference is valid.
    pub fn get(ptr: *const BootInfo) -> &'static Self {
        let bootinfo: &Self = unsafe { &*ptr };
        assert!(bootinfo.magic == BOOTINFO_MAGIC);
        assert!(bootinfo.end == BOOTINFO_END);
        assert!(bootinfo.size == size_of::<BootInfo>());

        bootinfo
    }
}
