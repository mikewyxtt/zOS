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