/*  loaded_image.rs - UEFI LoadedImage protocol
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

use super::EFIProtocol;
use super::{super::general::GUID, device_path::DevicePathProtocol};
use super::super::{SystemTable, bootservices::MemoryType};

// Each loaded image has an image handle that supports EFI_LOADED_IMAGE_PROTOCOL. When an image is
// started, it is passed the image handle for itself. The image can use the handle to obtain its relevant image data stored
// in the EFI_LOADED_IMAGE_PROTOCOL structure, such as its load options.
#[repr(C)]
pub struct LoadedImageProtocol {
    pub revision:           u32,
    _parent_handle:         *const usize,
    _system_table:          *const SystemTable,
    
    /// Device handle the EFI image was loaded from
    pub device_handle:      *const usize,
   
    pub file_path:          *const DevicePathProtocol,
    _reserved:              *const usize,
    _load_options_size:     u32,
    _load_options:          *const usize,
    _image_base:            *const usize,
    _image_size:            u64,
    _image_code_type:       MemoryType,
    _image_data_type:       MemoryType,
    _unload:                unsafe extern "efiapi" fn(),
}

impl LoadedImageProtocol {

    pub fn verify_revision(&self) -> bool {
        const REVISION: u32 = 0x1000;
        if self.revision == REVISION {
            true
        }

        else {
            false
        }
    }
}

impl EFIProtocol for LoadedImageProtocol {
    fn guid() -> GUID {
        GUID::new(0x5B1B31A1, 0x9562, 0x11d2, [0x8E,0x3F,0x00,0xA0,0xC9,0x69,0x72,0x3B])
    }
}