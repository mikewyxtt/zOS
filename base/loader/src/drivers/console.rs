/*  console.rs - Basic console driver
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


use core::fmt::{Write, Error, self};

/// Prints a formatted string with NO trailing newline to console output.
#[macro_export]
macro_rules! ldrprint {
    ($($arg:tt)*) => { ($crate::console::_ldrprint(format_args!($($arg)*))); }
}


/// Prints a formatted string with a trailing newline to console output.
#[macro_export]
macro_rules! ldrprintln {
    ($($arg:tt)*) => ($crate::ldrprint!("{}\n", format_args!($($arg)*)));
}


/// Clears the console
pub fn clear() {
    #[cfg(target_os = "uefi")] {
        super::uefi::console::clear();
    }
}


/// Print function that's used by the print macros
#[doc(hidden)]
pub fn _ldrprint(args: fmt::Arguments) {
    let mut writer = Writer::new();
    writer.write_fmt(args).unwrap();
}


/// Writer struct for fmt::Write to use. We don't need to do anything with it other than use it as a place to process the formatted text into, it is simple.
struct Writer;

impl Writer {
    /// Returns an empty Writer struct
    fn new() -> Self {
        Writer
    }

    /// Required for the fmt::Write trait. Writes a single byte to the console
    pub fn write_byte(&mut self, byte: u8) {
        #[cfg(target_os = "uefi")] {
            super::uefi::console::putc(byte.into());
        }
    }
}


/// In order to use the formatting stuff from the 'core' lib, we must provide the write_str implementation.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
