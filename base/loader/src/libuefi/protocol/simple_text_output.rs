/*  simple_text_output.rs - UEFI SimpleTextOutputProtocol implementation
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

use core::{ffi::c_void, sync::atomic::Ordering};
use super::super::SYSTEM_TABLE_PTR;


pub struct SimpleTextOutputProtocol {
    _reset:                     unsafe extern "efiapi" fn (*const Self, bool),
    _output_string:             unsafe extern "efiapi" fn (*const Self, *const u16),
    _test_string:               *const c_void,
    _query_mode:                *const c_void,
    _set_mode:                  unsafe extern "efiapi" fn (*const Self, usize) -> u32,
    _set_attribute:             *const c_void,
    _clear_screen:              *const c_void,
    _set_cursor_position:       *const c_void,
    _enable_cursor:             *const c_void,
    _mode:                      *const c_void
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode:               i32,
    pub mode:                   i32,
    pub attribute:              i32,
    pub cursor_column:          i32,
    pub cursor_row:             i32,
    pub cursor_visible:         bool
}


impl SimpleTextOutputProtocol {
    /// Returns a reference to SimpleTextOutputProtocol
    fn get() -> &'static Self {
        unsafe { &*(*(SYSTEM_TABLE_PTR.load(Ordering::SeqCst))).simple_text_output_protocol }
    }

    /// Resets the console
    pub fn reset() {
        unsafe { (Self::get()._reset)(Self::get(), false) };
    }

    /// Outputs a UTF-16 string to the UEFI console
    pub fn output_string(string: *const u16) {
        unsafe { (Self::get()._output_string)(Self::get(), string) };
    }

    /// Sets the output mode
    pub fn set_mode(mode_number: usize) -> u32 {
        unsafe { (Self::get()._set_mode)(Self::get(), mode_number) }
    }
}