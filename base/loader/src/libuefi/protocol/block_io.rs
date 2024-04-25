#![allow(dead_code)]

use core::ffi::c_void;
use super::super::GUID;

#[repr(C)]
pub struct BlockIOProtocol {
    pub revision:       u64,
    pub media:          *const BlockIOMedia,
    _reset:             *const c_void,
    _read_blocks:       unsafe extern "efiapi" fn (*const Self, u32, u64, usize, *const c_void) -> u32,
    _write_blocks:      *const c_void,
    _flush_blocks:      *const c_void,
}

impl BlockIOProtocol {
    /// Returns the BlockIOProtocol GUID
    pub const fn guid() -> GUID {
        GUID::new(0x964e5b21, 0x6459, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b])
    }

    /// Reads from the disk
    pub fn read_blocks<T>(&self, lba: u64, buffer_size: usize, buffer: *mut T) -> u32 {
        unsafe { (self._read_blocks)(self, (*self.media).media_id, lba, buffer_size, buffer as *const c_void) }
    }
}

#[repr(C)]
pub struct BlockIOMedia {
    pub media_id:                               u32,
    pub removable_media:                        bool,
    pub media_present:                          bool,
    pub logical_partition:                      bool,
    pub read_only:                              bool,
    pub write_caching:                          bool,
    pub block_size:                             u32,
    pub io_align:                               u32,
    pub last_block:                             u64,
    pub lowest_aligned_lba:                     u64,
    pub logical_blocks_per_physical_block:      u32,
    pub optimal_transfer_length_granularity:    u32
}