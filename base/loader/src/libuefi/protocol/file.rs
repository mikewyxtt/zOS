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

use alloc::vec;
use alloc::vec::Vec;

use crate::libuefi::GUID;


#[repr(C)]
pub struct File {
    pub revision:       u64,
    _open:              unsafe extern "C" fn(&Self, &&mut Self, *const u16, u64, u64),
    _close:             unsafe extern "C" fn(),
    _delete:            unsafe extern "C" fn(),
    _read:              unsafe extern "C" fn(&Self, usize, *const ()),
    _write:             unsafe extern "C" fn(),
    _get_position:      unsafe extern "C" fn(),
    _set_position:      unsafe extern "C" fn(),
    _get_info:          unsafe extern "C" fn(),
    _set_info:          unsafe extern "C" fn(),
    _flush:             unsafe extern "C" fn(),
    _open_ex:           unsafe extern "C" fn(),
    _read_ex:           unsafe extern "C" fn(),
    _write_ex:          unsafe extern "C" fn(),
    _flush_ex:          unsafe extern "C" fn(),
}

impl File {
    pub fn open(&self, file: &str, open_mode: u64, attr: Option<u64>) -> &Self {
        // let f: *mut *mut File = core::ptr::dangling_mut();
        let f: &mut File = unsafe { &mut(*core::ptr::dangling_mut()) };
        
        unsafe { 
            match attr {
                None => { (self._open)(&self, &f, encode_utf16(file).as_ptr(), open_mode, 0); }

                _ => { panic!("File creation is not supported in this UEFI implementation."); }
            }
            
            f
        }
    }

    /// Reads the entire file into a Vec<u8>
    pub fn read(&self) -> Vec<u8> {
        let buffer_size = 100;

        let buff: Vec<u8> = vec![0; 100];
        unsafe { (self._read)(self, buffer_size, buff.as_ptr().cast()); }
        buff
    }
}



pub fn encode_utf16(s: &str) -> Vec<u16> {
    let mut utf16str: Vec<u16> = Vec::new();
    for c in str::encode_utf16(s) {
        utf16str.push(c);
    }

    utf16str
}
