/*  graphics_output.rs - UEFI Graphics Output protocol
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
use super::EFIProtocol;
use crate::uuid::GUID;


// Each loaded image has an image handle that supports EFI_LOADED_IMAGE_PROTOCOL. When an image is
// started, it is passed the image handle for itself. The image can use the handle to obtain its relevant image data stored
// in the EFI_LOADED_IMAGE_PROTOCOL structure, such as its load options.
#[repr(C)]
pub struct GraphicsOutputProtocol {
    _query_mode:         unsafe extern "efiapi" fn(&Self, mode_number: u32, size_of_info: *mut usize, mode_info: *const *const ModeInfo) -> u32,
    _set_mode:           unsafe extern "efiapi" fn(&Self, u32) -> u32,
    _blt:                unsafe extern "efiapi" fn(),
    pub mode:            *const Mode,
}

impl GraphicsOutputProtocol {
    pub fn query_mode<'a>(&self, mode_number: u32) -> Result<(usize, &'a ModeInfo), u32> {
        let mut size_of_info: usize = 0;
        let mode_info: *const *const ModeInfo = ptr::dangling();

        let result = unsafe { (self._query_mode)(&self, mode_number, &mut size_of_info, mode_info) };

        if result == 0 {
            let mode_info = unsafe { &**mode_info };
            Ok((size_of_info, mode_info))
        }
        else {
            Err(result)
        }
    }

    pub fn set_mode(&self, mode_number: u32) -> Result<(), u32> {
        let result = unsafe { (self._set_mode)(self, mode_number) };

        if result == 0 {
            Ok(())
        }
        else {
            Err(result)
        }
    }
    
}

impl EFIProtocol for GraphicsOutputProtocol {
    fn guid() -> GUID {
        GUID::new(0x9042a9de, 0x23dc, 0x4a38, [0x96,0xfb,0x7a,0xde,0xd0,0x80,0x51,0x6a])
    }
}


#[repr(C)]
pub struct Mode {
    pub max_mode:       u32,
    pub mode:           u32,
    pub mode_info:      *const ModeInfo,
    pub size_of_info:   usize,
    pub fb_base:        *const u8,
    pub fb_size:        usize,
}

#[repr(C)]
pub struct ModeInfo {
    pub version:                u32,
    pub horizontal_resolution:  u32,
    pub vertical_resolution:    u32,
    pub pixel_format:           PixelFormat,
    pub pixel_info:             PixelBitmask,
    pub pixels_per_scanline:    usize,
}

#[repr(C)]
pub struct PixelBitmask {
    red_mask:       u32,
    green_mask:     u32,
    blue_mask:      u32,
    reserved_mask:  u32,
}

#[repr(C)]
pub enum PixelFormat {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax,
}