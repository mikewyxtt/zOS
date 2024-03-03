/*  hal/boot/bootinfo/src/i686.rs - i686 bootinfo struct.
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
 *  along with GRUB.  If not, see <http://www.gnu.org/licenses/>.
 */


 #![allow(dead_code)]

// pub mod arch {
    #[derive(Default)]
    #[repr(C)]
    pub struct ArchBootInfo {
        pub x: i32,
    }
    
    pub struct GlobalDescriptorTable {
        entries: [GdtEntry; 5],
    }
    
    #[repr(C, packed)]
    pub struct GdtEntry {
        limit_low: u16,
        base_low: u16,
        base_middle: u8,
        access: u8,
        granularity: u8,
        base_high: u8,
    }
    
    impl GdtEntry {
        pub fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
            let mut entry = GdtEntry {
                limit_low: (limit & 0xFFFF) as u16,
                base_low: (base & 0xFFFF) as u16,
                base_middle: ((base >> 16) & 0xFF) as u8,
                access,
                granularity,
                base_high: ((base >> 24) & 0xFF) as u8,
            };
    
            // Set the granularity bits in the GDT entry
            entry.granularity |= 0b1100_0000; // Set the flags
    
            entry
        }
    }
    
    impl GlobalDescriptorTable {
        pub fn new() -> Self {
            Self {
                entries: [
                    GdtEntry::new(0, 0, 0, 0),                  // Placeholder for null descriptor
                    GdtEntry::new(0, 0xffffffff, 0x9A, 0xCF),   // Code segment descriptor (supervisor)
                    GdtEntry::new(0, 0xffffffff, 0x92, 0xCF),   // Data segment descriptor (supervisor)
                    GdtEntry::new(0, 0, 0x9A, 0xCF),            // Placeholder for user mode code segment descriptor
                    GdtEntry::new(0, 0, 0x92, 0xCF),            // Placeholder for user mode data segment descriptor
                ],
            }
        }
    }
// }