/*  uuid.rs - Unique Identifier stuff
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

use alloc::{format, string::String};


#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8]
}

impl GUID {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        GUID {
            data1,
            data2,
            data3,
            data4
        }
    }


    pub fn new_from_string(guid: &str) -> GUID {

        // Break the string up into its data parts
        let (data1, rem) = guid.split_once('-').unwrap();
        let (data2, rem) = rem.split_once('-').unwrap();
        let (data3, rem) = rem.split_once('-').unwrap();
        let data4 = rem.replace('-', "");

        // Convert each part into an integer
        let data1 = u32::from_str_radix(data1, 16).unwrap();
        let data2 = u16::from_str_radix(data2, 16).unwrap();
        let data3 = u16::from_str_radix(data3, 16).unwrap();


        // Since the values are in hex we break the string 2 chars at a time and convert said chars into a hex value (u8)
        // There is probably a better way to do this but this works for now
        let mut d4: [u8; 8] = [0; 8];

        let mut data4 = data4.as_str();
        let mut rem = data4;
        for b in 0..8 {
            (data4, rem) = rem.split_at(2);
            d4[b] = u8::from_str_radix(data4, 16).unwrap();
        }

        GUID::new(data1, data2, data3, d4)
    }

    pub fn as_string(&self) -> String {
        format!("{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                            u32::from_le(self.data1), // UEFI spec states the first 3 values are encoded as little endian regardless of arch
                            u16::from_le(self.data2),
                            u16::from_le(self.data3),
                            self.data4[0],
                            self.data4[1],
                            self.data4[2],
                            self.data4[3],
                            self.data4[4],
                            self.data4[5],
                            self.data4[6],
                            self.data4[7])
    }
}