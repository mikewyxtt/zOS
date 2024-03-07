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

const CHIMERA_MEMORY_MAP_TYPE_AVAILABLE: u8 = 0;
const CHIMERA_MEMORY_MAP_TYPE_RESERVED: u8 = 1;


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
                        bootinfo.framebuffer = chimera::hal::boot::bootinfo::Framebuffer {
                            enabled: true,
                            addr: (*multiboot_fb_tag).common.framebuffer_addr as usize,
                            width: (*multiboot_fb_tag).common.framebuffer_width,
                            height: (*multiboot_fb_tag).common.framebuffer_height,
                            pitch: (*multiboot_fb_tag).common.framebuffer_pitch,
                            depth: ((*multiboot_fb_tag).common.framebuffer_bpp / 8) as u32,
                            size: (((*multiboot_fb_tag).common.framebuffer_width as u64) * ((*multiboot_fb_tag).common.framebuffer_height as u64) * ((*multiboot_fb_tag).common.framebuffer_bpp / 8) as u64) as u64,
                        };

                        // Since we have a framebuffer, initialize the console.
                        bootinfo.console = chimera::hal::boot::bootinfo::Console {
                            cursor_pos: 0,
                            line: 0,
                            max_chars: bootinfo.framebuffer.width / 8,
                            max_line: bootinfo.framebuffer.height / 16,
                        };
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
                /* To get the memory map from the bootloader we must first get a pointer to the Multiboot Memory Map Tag so we can find the memory entries at the end of it
                 *  Once we have the memory map tag, we can get a pointer to the first memory map entry
                 *  Now that we have a pointer to the first memory map entry, we can begin copying its fields into our bootinfo table. We will then iterate through each entry,
                 *  making sure not to go any further than the last entry by comparing the mmap entry pointer to the total size of the tag. If the memory map entry pointer is
                 *  longer than the tag at the beginning of the while loop, we have read all of the entries.
                 */
                unsafe {
                    // Create pointers to the multiboot2 memory map tag and the first multiboot2 memory map entry
                    let multiboot_mmap_tag: *const MultibootTagMmap = core::ptr::from_raw_parts(tag as *const _, (*tag).size as usize);
                    let mut multiboot_mmap_entry = &(*multiboot_mmap_tag).entries[0] as *const MultibootMemoryMap;

                    // Iterate through multiboot memory map entries
                    let mut i = 0;

                    while (multiboot_mmap_entry as usize) < (tag as usize + (*tag).size as usize) {
                        bootinfo.memory_info.total_physical_memory += (*multiboot_mmap_entry).len as usize;

                        // Determine entry type
                        let entry_type;
                        if (*multiboot_mmap_entry).type_ == MULTIBOOT_MEMORY_AVAILABLE {
                            bootinfo.memory_info.available_memory += (*multiboot_mmap_entry).len as usize;
                            entry_type = CHIMERA_MEMORY_MAP_TYPE_AVAILABLE;
                        }
                        else {
                            entry_type = CHIMERA_MEMORY_MAP_TYPE_RESERVED;
                        }

                        // Add memory map entry to bootinfo memory map table
                        bootinfo.memory_info.memory_map[i] = chimera::hal::boot::bootinfo::MemoryMapEntry {
                            base_address: (*multiboot_mmap_entry).addr as usize,
                            length: (*multiboot_mmap_entry).len as usize,
                            type_: entry_type,
                        };

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
