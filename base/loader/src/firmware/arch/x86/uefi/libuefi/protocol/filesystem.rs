/*  filesystem.rs - UEFI Simple Filesystem protocol
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

use crate::uuid::GUID;

use super::{file::EFI_File, EFIProtocol};

#[repr(C)]
pub struct SimpleFilesystem {
    pub revision:   u64,
    _open_volume:   unsafe extern "C" fn(&Self, &&EFI_File),
}

impl SimpleFilesystem {

    pub fn open_volume(&self) -> &EFI_File {
        unsafe { 
            let f: &&EFI_File = &&*(core::ptr::dangling_mut());
            (self._open_volume)(&self, f);

            &(**f)
        }
    }
}

impl EFIProtocol for SimpleFilesystem {
    fn guid() -> GUID {
        GUID::new(0x0964e5b22, 0x6459, 0x11d2, [0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b])
    }
}