// initializer.rs

#![no_std]
#![no_main]
#![feature(ptr_metadata)]


mod multiboot2;
mod bootinfo;
mod i386bootinfo;
mod console;
mod writer;

// remove if not debugging
mod debug_tools;
pub use debug_tools::*;
use core::fmt::Write;

use bootinfo::BootInfo;
use i386bootinfo::i386BootInfo;
use console::*;
use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn main(magic: u32, multiboot2_info_address: usize) {
    if magic != multiboot2::MULTIBOOT2_BOOTLOADER_MAGIC {
        // do something? hang for now...
        loop {}
    }

    // Create bootinfo tables, set all values to their defaults, then initialize them
    let mut bootinfo: BootInfo = BootInfo::default();
    let mut i386bootinfo: i386BootInfo = i386BootInfo::default();
    initialize_boot_info(&mut bootinfo, &mut i386bootinfo, multiboot2_info_address);


    // log values to console to check them
    early_log!(&mut bootinfo, "Multiboot 2 Info:");
    early_log!(&mut bootinfo, "\tMagic Number: 0x{:x}", magic);
    early_log!(&mut bootinfo, "\tMultiboot header addr: 0x{:x}\n", multiboot2_info_address);

    early_log!(&mut bootinfo, "Framebuffer Info:");
    early_log!(&mut bootinfo, "\tEnabled: {}", bootinfo.framebuffer.enabled);
    early_log!(&mut bootinfo, "\tAddress: 0x{:x}", bootinfo.framebuffer.addr);

    early_log!(&mut bootinfo, "\tResolution: {}x{}", bootinfo.framebuffer.width, bootinfo.framebuffer.height);
    early_log!(&mut bootinfo, "\tPitch: {} bytes", bootinfo.framebuffer.pitch);
    early_log!(&mut bootinfo, "\tDepth: {} bits", bootinfo.framebuffer.depth * 8);
    early_log!(&mut bootinfo, "\tSize: {} bytes\n", bootinfo.framebuffer.size);

    early_log!(&mut bootinfo, "Console Info:");
    early_log!(&mut bootinfo, "\tMax chars: {}", bootinfo.console.max_chars);
    early_log!(&mut bootinfo, "\tMax lines: {}", bootinfo.console.max_line);
    early_log!(&mut bootinfo, "\tCursor position: {}", bootinfo.console.cursor_pos);
    early_log!(&mut bootinfo, "\tCursor line: {}", bootinfo.console.line);
    early_log!(&mut bootinfo, "\tLog buffer size: {}\n", bootinfo.early_log_buffer.size);

    early_log!(&mut bootinfo, "Serial Port Info:");
    early_log!(&mut bootinfo, "\tEnabled: {}", bootinfo.serial.enabled);
    early_log!(&mut bootinfo, "\tUsing Port: 0x{:x}\n", bootinfo.serial.port);
    

    loop {}
}


fn initialize_boot_info(bootinfo: &mut BootInfo, i386bootinfo: &mut i386BootInfo, multiboot2_info_address: usize) {
    
    bootinfo.early_log_buffer.size = bootinfo.early_log_buffer.buffer.len();

    // Setup serial port
    bootinfo.serial.enabled = true;
    bootinfo.serial.port = 0x3F8;

    parse_multiboot_header(bootinfo, i386bootinfo, multiboot2_info_address);

}

pub fn parse_multiboot_header(bootinfo: &mut BootInfo, i386bootinfo: &mut i386BootInfo, multiboot2_info_address: usize) {
    i386bootinfo.x = 0; // bs entry to hide warning
    
    // Set default values
    use multiboot2::*;
    
    // pointer to first multiboot tag entry
    let mut tag: *const MultibootTag = (multiboot2_info_address + 8) as *const _;


    loop {
        unsafe {
            match (*tag).type_ {
                MULTIBOOT_TAG_TYPE_FRAMEBUFFER => {
                    
                    let fbtag: *const MultibootTagFramebuffer = core::ptr::from_raw_parts(tag as *const _, (*tag).size as usize);

                    if (*fbtag).common.framebuffer_type == 1 {
                        // Type of 1 means RGB, 2 means EGA text mode (unsupported), 0 means indexed color (unsupported)
                        bootinfo.framebuffer.enabled = true;
                        bootinfo.framebuffer.addr = (*fbtag).common.framebuffer_addr as usize;
                        bootinfo.framebuffer.width = (*fbtag).common.framebuffer_width;
                        bootinfo.framebuffer.height = (*fbtag).common.framebuffer_height;
                        bootinfo.framebuffer.pitch = (*fbtag).common.framebuffer_pitch;
                        bootinfo.framebuffer.depth = ((*fbtag).common.framebuffer_bpp / 8) as u32;
                        bootinfo.framebuffer.size = (bootinfo.framebuffer.width as u64 * bootinfo.framebuffer.height as u64 * bootinfo.framebuffer.depth as u64) as u64;
                        
    
                        // // Since we have a framebuffer, initialize the console.
                        bootinfo.console.cursor_pos = 0;
                        bootinfo.console.line = 0;
                        bootinfo.console.max_chars = bootinfo.framebuffer.width / 8;
                        bootinfo.console.max_line = bootinfo.framebuffer.height / 16;
                    }
                }

                MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME => {
                    //serial_log!("Found MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME");
                }

                MULTIBOOT_TAG_TYPE_END => {
                    break;
                }

                // Handle tag types we don't care about
                _ => {
                }
            }
            
            // Index the tag pointer (tag.size) number of bytes forward, ensuring it is 8 byte aligned as per Multiboot 2 spec.
            tag = (tag as usize + ((*tag).size as usize + 7) & !7) as *const _;
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { debug_tools::set_eax(0xBad0Deed); }
    serial_log!("{}", _info);
    loop {}
}
