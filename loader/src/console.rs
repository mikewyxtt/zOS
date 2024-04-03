use crate::uefi;
use core::fmt::{Write, Error, self};


/// Prints a formatted string with NO trailing newline to console output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { ($crate::console::_print(format_args!($($arg)*))); }
}


/// Prints a formatted string with a trailing newline to console output.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ($crate::print!("{}\n\r", format_args!($($arg)*)));
}


/// Clears the console
pub fn reset() {
    uefi::SimpleTextOutputProtocol::reset();
}


/// Print function that's used by the print macros
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
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
        
        // UEFI requires UTF-16 string literals. So, we simply create a 16 bit array with the char and '\0' so it thinks it is the end of the string, then call the UEFI output_string function with a pointer to the array. It works...
        let utf16_str: [u16; 2] = [byte.into(), b'\0'.into()];
        uefi::SimpleTextOutputProtocol::output_string(utf16_str.as_ptr());
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
