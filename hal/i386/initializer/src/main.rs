/*  hal/i386/initializer/src/main.rs - initializer main
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

#![no_std]
#![no_main]
#![feature(ptr_metadata)]


mod multiboot2;
mod initbootinfo;
mod gdt;

use core::panic::PanicInfo;
use zOS::hal::boot::bootinfo::BootInfo;
use zOS::hal::boot::bootinfo::i686::ArchBootInfo;
use zOS::log::*;


#[no_mangle]
pub extern "C" fn main(magic: u32, multiboot2_info_address: usize) {
    if magic != multiboot2::MULTIBOOT2_BOOTLOADER_MAGIC {
        // do something? hang for now...
        loop {}
    }

    // Create bootinfo tables, set all values to their defaults, then initialize them
    let mut bootinfo: BootInfo = BootInfo::default();
    let mut archbootinfo: ArchBootInfo = ArchBootInfo::default();
    initbootinfo::initialize(&mut bootinfo, &mut archbootinfo, multiboot2_info_address);

    // log values to console to check them
    let bootinfo_clone = bootinfo.clone();
    
    early_log!(&mut bootinfo, "Multiboot 2 Info:");
    early_log!(&mut bootinfo, "\tMagic Number: 0x{:x}", magic);
    early_log!(&mut bootinfo, "\tBoot Information struct Address: 0x{:x}\n", multiboot2_info_address);

    early_log!(&mut bootinfo, "Framebuffer Info:");
    early_log!(&mut bootinfo, "\tEnabled: {}", bootinfo_clone.framebuffer.enabled);
    early_log!(&mut bootinfo, "\tAddress: 0x{:x}", bootinfo_clone.framebuffer.addr);
    early_log!(&mut bootinfo, "\tResolution: {}x{}", bootinfo_clone.framebuffer.width, bootinfo_clone.framebuffer.height);
    early_log!(&mut bootinfo, "\tPitch: {} bytes", bootinfo_clone.framebuffer.pitch);
    early_log!(&mut bootinfo, "\tDepth: {} bits", bootinfo_clone.framebuffer.depth * 8);
    early_log!(&mut bootinfo, "\tSize: {} bytes\n", bootinfo_clone.framebuffer.size);

    early_log!(&mut bootinfo, "Console Info:");
    early_log!(&mut bootinfo, "\tMax chars: {}", bootinfo_clone.console.max_chars);
    early_log!(&mut bootinfo, "\tMax lines: {}", bootinfo_clone.console.max_line);
    early_log!(&mut bootinfo, "\tLog buffer size: {}\n", bootinfo_clone.early_log_buffer.size);

    early_log!(&mut bootinfo, "Serial Port Info:");
    early_log!(&mut bootinfo, "\tEnabled: {}", bootinfo_clone.serial.enabled);
    early_log!(&mut bootinfo, "\tUsing Port: 0x{:x}\n", bootinfo_clone.serial.port);

    early_log!(&mut bootinfo, "Memory Info:");
    early_log!(&mut bootinfo, "\tPhysical memory: {}KB", bootinfo_clone.memory_info.total_physical_memory / 1024);
    early_log!(&mut bootinfo, "\tAvailable memory: {}KB", bootinfo_clone.memory_info.available_memory / 1024);
    early_log!(&mut bootinfo, "\tMemory map entries: {}", bootinfo_clone.memory_info.memory_map_entries);

    
    // Print available entries first
    let mut i = 0;
    for _ in 0..bootinfo.memory_info.memory_map_entries {
        if bootinfo.memory_info.memory_map[i].type_ == 0 {
            early_log!(&mut bootinfo, "\tAvailable Entry:");
            early_log!(&mut bootinfo, "\t\tBase Address: 0x{:x}", bootinfo_clone.memory_info.memory_map[i].base_address);
            early_log!(&mut bootinfo, "\t\tLength: {}KB", bootinfo_clone.memory_info.memory_map[i].length / 1024);
        }
        i += 1;
    }

    // Print reserved entries
    let mut i = 0;
    for _ in 0..bootinfo.memory_info.memory_map_entries {
        if bootinfo.memory_info.memory_map[i].type_ == 1 {
            early_log!(&mut bootinfo, "\tReserved Entry:");
            early_log!(&mut bootinfo, "\t\tBase Address: 0x{:x}", bootinfo_clone.memory_info.memory_map[i].base_address);
            early_log!(&mut bootinfo, "\t\tLength: {}KB", bootinfo_clone.memory_info.memory_map[i].length / 1024);
        }
        i += 1;
    }

    // Replace the GDT that the bootloader gave us with ours
    gdt::setup_gdt(&mut archbootinfo);

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use debugtools::*;
    serial_log!("{}", _info);
    unsafe { zOS::debug::debugtools::set_eax(0xBadDeed); }
    loop {}
}
