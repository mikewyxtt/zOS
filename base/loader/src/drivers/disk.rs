/*  disk.rs - Basic disk driver
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


use core::mem::size_of;
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use alloc::string::String;

use crate::libuefi::GUID;



/// Reads data from the disk as Box<T>
pub fn read_bytes_into_box<T>(partition: GUID, lba: u64, count: usize) -> Box<T> {
    assert!(count <= size_of::<T>());

    let mut t: Box<T> = unsafe { Box::new(core::mem::zeroed()) };

    let buffer = read_bytes(partition, lba, count).unwrap();
    unsafe { core::ptr::copy(buffer.as_ptr(), (t.as_mut() as *mut T).cast(), count); }

    t
}






/// Reads data from the disk into T
pub fn read_bytes_into<T>(partition: GUID, lba: u64, count: usize, buffer: &mut T) {
    assert!(count <= core::mem::size_of_val(buffer));
    unsafe { read_bytes_raw(partition, lba, count, (buffer as *mut T).cast()).unwrap(); }
}






/// Reads bytes from the disk into a Vec<u8>
/// 
/// count: Number of bytes to read
/// lba: Logical block address, which logical block to start reading from
/// device: Deivce to read from. e.g disk0s1
/// buffer: Buffer to fill with bytes
pub fn read_bytes(partition: GUID, lba: u64, count: usize) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = vec![0; count];

    unsafe {
        match read_bytes_raw(partition, lba, count, buffer.as_mut_ptr()) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(error)
        }
    }
}








/// Reads bytes from the disk into a buffer. 'buffer' is a raw ptr, and is unsafe as the boundaries cannot be checked.
/// 
/// count: Number of bytes to read
/// lba: Logical block address, which logical block to start reading from
/// device: Deivce to read from. e.g disk0s1
/// buffer: Buffer to fill with bytes
pub unsafe fn read_bytes_raw(partition: GUID, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    super::uefi::disk::read_bytes_raw(partition, lba, count, buffer)
}



/// Searches for block devices and returns a Vector of BlockDevice structs
fn probe_disks() {
    super::uefi::disk::init()
}


pub fn init(){
    probe_disks();
}