// debug_tools.rs

//use crate::writer::*;

#![allow(dead_code)]

use core::arch::asm;
//use core::fmt::Write;

// Define a macro for printing to serial
#[macro_export]
macro_rules! serial_log {
    ($($arg:tt)*) => {{
        // Create a fixed-size buffer for the formatted string
        let mut buffer = [0u8; 1024];
        let mut cursor = $crate::writer::create_cursor(&mut buffer[..]);

        // Write the formatted string to the buffer
        let _ = write!(&mut cursor, "{}\n", core::format_args!($($arg)*));

        // Call the custom print function with the buffer content
        $crate::print_to_serial(core::str::from_utf8(&buffer).unwrap());
    }}
}


#[inline(always)]
unsafe fn write_byte(port: u16, data: u8) {
    asm!(   "out dx, al",
            in("al") data,
            in("dx") port);
}

pub fn print_to_serial(text: &str) {
    for c in text.chars() {
        if c == '\n' {
            unsafe { write_byte(0x3F8, b'\r' as u8); }
        }
        unsafe { write_byte(0x3F8, c as u8); }
    }
}


/// Sets 'EAX' register to 'value'
pub unsafe fn set_eax(value: u32) {
    core::arch::asm!(
        "mov eax, {}",
        in(reg) value
        );
}