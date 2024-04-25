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

pub struct MemoryMap {
    start:      usize,
    size:       usize,
    _type:      u8,
}

pub struct Extension {
    pub name:   [char; 24],
    pub addr:   usize,
    pub size:   usize,
}

#[repr(C)]
pub struct BootInfo<T> {
    pub magic:          u16,
    pub version:        [char; 8],
    pub size:           usize,

    // Framebuffer info
    pub fb_enabled:     bool,
    pub fb_addr:        usize,
    pub fb_width:       u32,
    pub fb_height:      u32,
    pub fb_pitch:       u32,
    pub fb_depth:       u32,
    pub fb_size:        u64,

    pub memory_map:     [MemoryMap; 24],
    pub extensions:     [Extension; 32],

    // Architecture specific stuff
    pub arch_info:      T,

    // Value that can be checked to ensure struct boundaries are correct
    pub end:            u16,
}