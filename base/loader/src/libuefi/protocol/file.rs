/*  file.rs - UEFI File protocol
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

use alloc::vec::Vec;

use crate::libuefi::GUID;


#[allow(non_camel_case_types)]
#[repr(C)]
pub struct EFI_File {
    pub revision:       u64,
    _open:              unsafe extern "C" fn(&Self, &mut &mut Self, *const u16, u64, u64) -> u32,
    _close:             unsafe extern "C" fn(&Self),
    _delete:            unsafe extern "C" fn(),
    _read:              unsafe extern "C" fn(&Self, &mut usize, *const u8) -> u32,
    _write:             unsafe extern "C" fn(),
    _get_position:      unsafe extern "C" fn(),
    _set_position:      unsafe extern "C" fn(),
    _get_info:          unsafe extern "C" fn(&Self, &GUID, &usize, *const usize) -> u32,
    _set_info:          unsafe extern "C" fn(),
    _flush:             unsafe extern "C" fn(),
    _open_ex:           unsafe extern "C" fn(),
    _read_ex:           unsafe extern "C" fn(),
    _write_ex:          unsafe extern "C" fn(),
    _flush_ex:          unsafe extern "C" fn(),
}


impl EFI_File {
    pub fn open(&self, file: &str, open_mode: u64, attr: Option<u64>) -> &Self {
        assert_eq!(attr, None, "File creation is not supported in this UEFI implementation.");

        let f: &mut &mut EFI_File = unsafe { &mut(*core::ptr::dangling_mut()) };
        
        let utf16_str: Vec<u16> = file.encode_utf16().collect();

        unsafe {
            let status = (self._open)(&self, f, utf16_str.as_ptr(), open_mode, 0);
            match status {
                0 => { return &(**f); }

                _ => { panic!("EFI ERROR: {}", status); }
            }
        }
    }

    pub fn close(&self) {
        unsafe { ((self._close)(self)); }
    }


    /// Reads the entire file into a Vec<T>
    pub unsafe fn read<T>(&self, count: &mut usize, buffer: *mut T) {

        let result = unsafe { (self._read)(&self, count, buffer.cast()) };

        match result {
            0 => { }

            _ => { panic!("EFI Error: {}", result) }
        }
    }


    pub fn get_info(&self, info_type: GUID) -> &FileInfo {
        let buffer_size = 102;
        let file_info: &FileInfo = unsafe { &*core::ptr::dangling() };
        let result = unsafe { (self._get_info)(&self, &info_type, &buffer_size, (file_info as *const FileInfo).cast()) };

        match result {
            0 => { file_info }

            _ => { panic!("EFI Error: {}", result) }
        }
    }
}

#[repr(C)]
pub struct FileInfo {
    pub size:           u64,
    pub file_size:      u64,
    pub phys_size:      u64,
    pub create_time:    [u8; 14], // EFI_TIME
    pub last_accessed:  [u8; 14], // ^
    pub last_modified:  [u8; 14], // ^
    pub attribute:      u64,
    pub filename:       [u16; 16],
}

impl FileInfo {
    pub const fn guid() -> GUID {
        GUID::new(0x09576e92,0x6d3f,0x11d2,[0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b])
    }
}
