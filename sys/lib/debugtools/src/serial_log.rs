// sys/lib/debugtools/src/serial_log.rs


use core::fmt::{Write, Error, self};


/// Prints a formatted string with NO trailing newline to serial output. It defaults to 0x3F8..
#[macro_export]
macro_rules! serial_log_plain {
    ($($arg:tt)*) => ($crate::serial_log::_serial_log(format_args!($($arg)*)));
}


/// Prints a formatted string with a trailing newline to serial output. It defaults to 0x3F8..
#[macro_export]
macro_rules! serial_log {
    () => ($crate::serial_log_plain!("\n"));
    ($($arg:tt)*) => ($crate::serial_log_plain!("{}\n", format_args!($($arg)*)));
}


/// Serial logging function, used by serial_log! macro. Creates a temporary text buffer and writes the formatted text to it.
/// The text is then immediately output to the serial port, then the buffer is discarded.
#[doc(hidden)]
pub fn _serial_log(args: fmt::Arguments) {
    let mut serial_writer = SerialWriter::new();
    serial_writer.write_fmt(args).unwrap();
}


/// Writer struct for fmt::Write to use. We don't need to do anything with it other than use it as a place to process the formatted text into, it is simple.
pub struct SerialWriter {
    buffer: [u8; 100],
    // position: usize,
}



impl SerialWriter {
    
    /// Returns an empty SerialWriter struct
    fn new() -> Self {
        SerialWriter {
            buffer: [b'\0'; 100],
            // position: 0,
        }
    }

    /// Writes a single byte to the serial port
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            unsafe { hal::io::write_byte(0x3F8, b'\r' as u8); }
        }
        unsafe { hal::io::write_byte(0x3F8, byte); }

        // self.position += 1;
    }
}


/// In order to use the formatting stuff from the 'core' lib, we must provide the write_str implementation.
impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for byte in s.chars() {
            self.write_byte(byte as u8);
        }
        Ok(())
    }
}