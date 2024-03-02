// bootinfo.rs

use crate::lib_bootinfo::BootInfo;
use crate::i386bootinfo::i386BootInfo;
use crate::multiboot2::*;

pub fn initialize(bootinfo: &mut BootInfo, i386bootinfo: &mut i386BootInfo, multiboot2_info_address: usize) {
    
    bootinfo.early_log_buffer.size = bootinfo.early_log_buffer.buffer.len();


    // Setup serial port if debugging is enabled
    #[cfg(feature = "serial_debug")]
    {
        bootinfo.serial.enabled = true;
        bootinfo.serial.port = 0x3F8;
    }

    parse_multiboot_header(bootinfo, i386bootinfo, multiboot2_info_address);

}

pub fn parse_multiboot_header(bootinfo: &mut BootInfo, i386bootinfo: &mut i386BootInfo, multiboot2_info_address: usize) {
    i386bootinfo.x = 0; // bs entry to hide warning
    
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