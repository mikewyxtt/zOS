/*  device_path.rs - UEFI DevicePath protocol implementation
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


use super::super::GUID;


#[repr(C, packed)]
pub struct DevicePathProtocol {
    pub _type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
}

impl DevicePathProtocol {
    pub const fn guid() -> GUID {
        GUID::new(0x09576e91,0x6d3f,0x11d2,[0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b])
    }

    pub fn next(&self) -> &Self {
        let next_node = unsafe { &*(((self as *const Self as usize) + self.length[0] as usize + self.length[1] as usize) as *const Self) };

        // Check the node type is sane to ensure we don't return a bad reference
        match next_node._type {
            0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x7F => { next_node }

            _ => { panic!("Next EFI Device Path node is invalid. Node type: 0x{:02X}", next_node._type); }
        }
    }
}

#[repr(C, packed)]
pub struct ACPIDevicePath {
    pub _type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
    pub hid: u32,
    pub uid: u32,
}

#[repr(C, packed)]
pub struct PCIDevicePath {
    pub _type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
    pub function: u8,
    pub device: u8,
}

#[repr(C, packed)]
pub struct HardDriveDevicePath {
    pub _type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
    pub partition_number: u32,
    pub partition_start: u64,
    pub partition_size: u64,
    pub partition_sig: GUID,
    pub partition_format: u8,
    pub sig_type: u8,
}
