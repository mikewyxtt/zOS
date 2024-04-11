#![allow(dead_code)]

use core::ffi::c_void;

use crate::uefi::bootservices::MemoryType;

use super::{super::general::GUID, device_path::DevicePathProtocol};
use super::super::SystemTable;

// Each loaded image has an image handle that supports EFI_LOADED_IMAGE_PROTOCOL. When an image is
// started, it is passed the image handle for itself. The image can use the handle to obtain its relevant image data stored
// in the EFI_LOADED_IMAGE_PROTOCOL structure, such as its load options.
#[repr(C, packed)]
pub struct LoadedImageProtocol {
    pub revision:           u32,
    _parent_handle:         *const usize,
    _system_table:          *const SystemTable,
    
    /// Device handle the EFI image was loaded from
    pub device_handle:      *const usize,
   
    _file_path:             *const DevicePathProtocol,
    _reserved:              *const c_void,
    _load_options_size:     u32,
    _load_options:          *const c_void,
    _image_base:            *const c_void,
    _image_size:            u64,
    _image_code_type:       MemoryType,
    _image_data_type:       MemoryType,
    _unload:                unsafe extern "efiapi" fn(),
}

impl LoadedImageProtocol {
    /// Returns GUID for the LoadedImageProtocol
    pub const fn guid() -> GUID {
        GUID::new(0x5B1B31A1, 0x9562, 0x11d2, [0x8E,0x3F,0x00,0xA0,0xC9,0x69,0x72,0x3B])
    }

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