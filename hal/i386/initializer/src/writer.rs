// writer.rs

use core::fmt::Write;

// Define a cursor type
pub struct Cursor<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    // Create a new cursor instance
    fn new(buffer: &'a mut [u8]) -> Self {
        Cursor {
            buffer,
            position: 0,
        }
    }
}

pub fn create_cursor(buffer: &mut [u8]) -> Cursor {
    Cursor::new(buffer)
}

impl<'a> Write for Cursor<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if self.position >= self.buffer.len() {
                return Err(core::fmt::Error);
            }
            self.buffer[self.position] = byte;
            self.position += 1;
        }
        Ok(())
    }
}