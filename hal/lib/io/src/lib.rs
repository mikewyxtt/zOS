// hal/lib/io/lib.rs

#![no_std]
#![allow(dead_code)]

/// Writes a single byte to 'port'
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
pub unsafe fn write_byte(port: u16, data: u8) {
    core::arch::asm!(   "out dx, al",
            in("al") data,
            in("dx") port);
}

// Reads a single byte from 'port'
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(unreachable_code)]
#[inline(always)]
pub unsafe fn read_byte(__port: u16) -> u8{
    panic!("read_byte() not implemented.");
    0
}