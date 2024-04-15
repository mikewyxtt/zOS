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
    pub uid: u32
}
