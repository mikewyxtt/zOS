/*  general.rs - misc UEFI stuff
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

use core::{ffi::c_void, sync::atomic::{AtomicPtr, Ordering}};

use super::bootservices::BootServices;
use super::protocol::simple_text_output::SimpleTextOutputProtocol;

pub static SYSTEM_TABLE_PTR: AtomicPtr<SystemTable> = AtomicPtr::new(core::ptr::dangling_mut());
pub static IMAGE_HANDLE: AtomicPtr<usize> = AtomicPtr::new(core::ptr::null_mut());



#[repr(C)]
pub struct TableHeader {
    pub signature:      u64,
    pub revision:       u32,
    pub header_size:    u32,
    pub crc32:          u32,
    reserved:           u32
}


#[repr(C)]
pub struct SystemTable {
    pub header:                                     TableHeader,
    pub firmware_vendor:                            *const u16,
    pub firmware_revision:                          u32,
    pub console_in_handle:                          *const c_void,
    pub simple_text_input_protocol:                 *const c_void,
    pub console_out_handle:                         *const c_void,
    pub simple_text_output_protocol:                *const SimpleTextOutputProtocol,
    pub standard_error_handle:                      *const c_void,
    pub std_error:                                  *const c_void,
    pub runtime_services:                           *const c_void,
    pub boot_services:                              *const BootServices,
    pub number_of_table_entries:                    usize,
    pub configuration_table:                        *const c_void
}



/// Initializes the pointer to the system table
pub fn init(image_handle: *const usize, system_table: *const SystemTable) {
    // TODO: add a check validating these pointers
    SYSTEM_TABLE_PTR.store(system_table.cast_mut(), Ordering::SeqCst);
    IMAGE_HANDLE.store(image_handle.cast_mut(), Ordering::SeqCst);
}

