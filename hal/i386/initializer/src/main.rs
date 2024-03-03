/*  hal/i386/initializer/src/main.rs - initializer main
 *
 *  chimera  --  Advanced *NIX System
 *  Copyright (C) 2024  Free Software Foundation, Inc.
 *
 *  chimera is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  chimera is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with GRUB. If not, see <http://www.gnu.org/licenses/>.
 */

#![no_std]
#![no_main]
#![feature(ptr_metadata)]


mod multiboot2;
mod initbootinfo;

use core::panic::PanicInfo;
use chimera::boot::bootinfo::BootInfo;
use chimera::boot::bootinfo::arch::i386BootInfo;


#[no_mangle]
pub extern "C" fn main(magic: u32, multiboot2_info_address: usize) {
    if magic != multiboot2::MULTIBOOT2_BOOTLOADER_MAGIC {
        // do something? hang for now...
        loop {}
    }

    // Create bootinfo tables, set all values to their defaults, then initialize them
    let mut bootinfo: BootInfo = BootInfo::default();
    let mut i386bootinfo: i386BootInfo = i386BootInfo::default();
    initbootinfo::initialize(&mut bootinfo, &mut i386bootinfo, multiboot2_info_address);


    // log values to console to check them
    // early_log!(&mut bootinfo, "Multiboot 2 Info:");
    // early_log!(&mut bootinfo, "\tMagic Number: 0x{:x}", magic);
    // early_log!(&mut bootinfo, "\tBoot Information struct Address: 0x{:x}\n", multiboot2_info_address);

    // early_log!(&mut bootinfo, "Framebuffer Info:");
    // early_log!(&mut bootinfo, "\tEnabled: {}", bootinfo.framebuffer.enabled);
    // early_log!(&mut bootinfo, "\tAddress: 0x{:x}", bootinfo.framebuffer.addr);

    // early_log!(&mut bootinfo, "\tResolution: {}x{}", bootinfo.framebuffer.width, bootinfo.framebuffer.height);
    // early_log!(&mut bootinfo, "\tPitch: {} bytes", bootinfo.framebuffer.pitch);
    // early_log!(&mut bootinfo, "\tDepth: {} bits", bootinfo.framebuffer.depth * 8);
    // early_log!(&mut bootinfo, "\tSize: {} bytes\n", bootinfo.framebuffer.size);

    // early_log!(&mut bootinfo, "Console Info:");
    // early_log!(&mut bootinfo, "\tMax chars: {}", bootinfo.console.max_chars);
    // early_log!(&mut bootinfo, "\tMax lines: {}", bootinfo.console.max_line);
    // early_log!(&mut bootinfo, "\tCursor position: {}", bootinfo.console.cursor_pos);
    // early_log!(&mut bootinfo, "\tCursor line: {}", bootinfo.console.line);
    // early_log!(&mut bootinfo, "\tLog buffer size: {}\n", bootinfo.early_log_buffer.size);

    // early_log!(&mut bootinfo, "Serial Port Info:");
    // early_log!(&mut bootinfo, "\tEnabled: {}", bootinfo.serial.enabled);
    // early_log!(&mut bootinfo, "\tUsing Port: 0x{:x}\n", bootinfo.serial.port);
    
    use debugtools::*;
    serial_log!("Hi! Regular number: {} Hex number: 0x{:x}", 50, 10);
    serial_log!("Hi! Regular number: {} Hex number: 0x{:x}", 50, 10);

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use debugtools::*;
    serial_log!("{}", _info);
    unsafe { chimera::debug_tools::set_eax(0xBad0Deed); }
    loop {}
}
