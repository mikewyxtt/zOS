/*  hal/i386/initializer/src/initbootinfo.rs - Initialize bootinfo tables
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

use chimera::hal::boot::bootinfo::BootInfo;
use chimera::hal::boot::bootinfo::i686::ArchBootInfo;
use crate::multiboot2::*;


pub fn initialize(bootinfo: &mut BootInfo, archbootinfo: &mut ArchBootInfo, multiboot2_info_address: usize) {
    
    bootinfo.early_log_buffer.size = bootinfo.early_log_buffer.buffer.len();


    // Setup serial port if debugging is enabled
    #[cfg(feature = "serial_debug")]
    {
        bootinfo.serial.enabled = true;
        bootinfo.serial.port = 0x3F8;
    }

    parse_multiboot_header(bootinfo, archbootinfo, multiboot2_info_address);
}


pub fn parse_multiboot_header(bootinfo: &mut BootInfo, archbootinfo: &mut ArchBootInfo, multiboot2_info_address: usize) {
    archbootinfo.x = 0;
    
    // pointer to first multiboot tag entry
    let mut tag: *const MultibootTag = (multiboot2_info_address + 8) as *const _;


    loop {
        match unsafe { (*tag).type_ } {
            MULTIBOOT_TAG_TYPE_FRAMEBUFFER => {
                unsafe {
                    let multiboot_fb_tag: *const MultibootTagFramebuffer = core::ptr::from_raw_parts(tag as *const _, (*tag).size as usize);

                    if (*multiboot_fb_tag).common.framebuffer_type == 1 {
                        // Type of 1 means RGB, 2 means EGA text mode (unsupported), 0 means indexed color (unsupported)
                        bootinfo.framebuffer.enabled = true;
                        bootinfo.framebuffer.addr = (*multiboot_fb_tag).common.framebuffer_addr as usize;
                        bootinfo.framebuffer.width = (*multiboot_fb_tag).common.framebuffer_width;
                        bootinfo.framebuffer.height = (*multiboot_fb_tag).common.framebuffer_height;
                        bootinfo.framebuffer.pitch = (*multiboot_fb_tag).common.framebuffer_pitch;
                        bootinfo.framebuffer.depth = ((*multiboot_fb_tag).common.framebuffer_bpp / 8) as u32;
                        bootinfo.framebuffer.size = (bootinfo.framebuffer.width as u64 * bootinfo.framebuffer.height as u64 * bootinfo.framebuffer.depth as u64) as u64;

                    
                        // Since we have a framebuffer, initialize the console.
                        bootinfo.console.cursor_pos = 0;
                        bootinfo.console.line = 0;
                        bootinfo.console.max_chars = bootinfo.framebuffer.width / 8;
                        bootinfo.console.max_line = bootinfo.framebuffer.height / 16;
                    }
                }
            }

            MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME => {
                //serial_log!("Found MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME");
            }

            MULTIBOOT_TAG_TYPE_END => {
                break;
            }

            MULTIBOOT_TAG_TYPE_MMAP => {
                unsafe {
                    let multiboot_mmap_tag: *const MultibootTagMmap = core::ptr::from_raw_parts(tag as *const _, (*tag).size as usize);
                    let mut multiboot_mmap_entry = &(*multiboot_mmap_tag).entries[0] as *const MultibootMemoryMap;

                    let mut i = 0;

                    while (multiboot_mmap_entry as usize) < (tag as usize + (*tag).size as usize) {
                        bootinfo.memory_info.total_physical_memory += (*multiboot_mmap_entry).len as usize;

                        if (*multiboot_mmap_entry).type_ == MULTIBOOT_MEMORY_AVAILABLE {
                            bootinfo.memory_info.available_memory += (*multiboot_mmap_entry).len as usize;
                            bootinfo.memory_info.memory_map[i].base_address = (*multiboot_mmap_entry).addr as usize;
                            bootinfo.memory_info.memory_map[i].length = (*multiboot_mmap_entry).len as usize;
                            bootinfo.memory_info.memory_map[i].type_ = 0;
                        }
                        else {
                            bootinfo.memory_info.memory_map[i].base_address = (*multiboot_mmap_entry).addr as usize;
                            bootinfo.memory_info.memory_map[i].length = (*multiboot_mmap_entry).len as usize;
                            bootinfo.memory_info.memory_map[i].type_ = 1;
                        }

                        i += 1;
                        multiboot_mmap_entry = &(*multiboot_mmap_tag).entries[i] as *const MultibootMemoryMap;
                    }
                    bootinfo.memory_info.memory_map_entries = i as u16;
                }
            }

            // Handle tag types we don't care about
            _ => {
            }
        }
        
        // Index the tag pointer (tag.size) number of bytes forward, ensuring it is 8 byte aligned as per Multiboot 2 spec.
        tag = unsafe { (tag as usize + ((*tag).size as usize + 7) & !7) as *const _ };
    }
}
