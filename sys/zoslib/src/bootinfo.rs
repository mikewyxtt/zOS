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

pub const BOOTINFO_MAGIC: u16 = 0xFAFA;

#[repr(C)]
pub struct MemoryMap {
    pub start:      usize,
    pub size:       usize,
    pub _type:      u8,
}

#[repr(C)]
pub struct Extension {
    pub name:       [char; 24],
    pub addr:       usize,
    pub size:       usize,
}

#[repr(C)]
pub struct Framebuffer {
    pub enabled:     bool,
    pub addr:        usize,
    pub width:       u32,
    pub height:      u32,
    pub pitch:       u32,
    pub depth:       u32,
    pub size:        u64,
}

#[repr(C)]
pub struct BootInfo<T> {
    pub magic:          u16,
    pub version:        [char; 8],
    pub size:           usize,

    pub cmdline:        [char; 50],             // Boot command line
    pub framebuffer:    Framebuffer,
    pub memory_map:     [MemoryMap; 24],
    pub extensions:     [Extension; 32],
    pub arch_info:      T,                      // Architecture specific stuff

    pub end:            u16,                    // Value that can be checked to ensure struct boundaries are correct
}


impl<T> BootInfo<T> {
    pub fn new() -> Self {
        let v = env!("zOS_VERSION");
        let mut version = ['\0'; 8];

        for (i, byte) in v.bytes().enumerate() {
            version[i] = byte.into();
        }

        Self {
            magic:          BOOTINFO_MAGIC,
            version:        version,
            size:           core::mem::size_of::<Self>(),
            cmdline:        ['\0'; 50],
            framebuffer:    unsafe { core::mem::zeroed() },
            memory_map:     unsafe { core::mem::zeroed() },
            extensions:     unsafe { core::mem::zeroed() },
            arch_info:      unsafe { core::mem::zeroed() },
            end:            0xFF55,
        }
    }
}