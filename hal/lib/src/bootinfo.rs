/*  hal/lib/src/bootinfo.rs -- Bootinfo table definitions
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



 /*
  *
  *
  * Universal BootInfo table
  *
  */

 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct BootInfo {
     pub early_log_buffer: EarlyLogBuffer,
     pub framebuffer: Framebuffer,
     pub console: Console,
     pub serial: Serial,
     pub critical_components: CriticalComponents,
     pub memory_info: MemoryInfo,
     pub cpu_info: CPUInfo,
     //pub params: [char; 50],
     pub config: Config,
 }
 
 #[derive(Copy, Clone)]
 #[repr(C)]
 pub struct EarlyLogBuffer {
     pub size: usize,
     pub index: u16,
     pub last_flush_index: u16,
     pub buffer: [char; 6144],
 }
 
 impl Default for EarlyLogBuffer {
     fn default() -> Self {
         // Initialize size, index, and last_flush_index to 0
         let size = 0;
         let index = 0;
         let last_flush_index = 0;
 
         // Initialize buffer to contain '\0' characters
         let buffer = ['\0'; 6144];
 
         // Construct EarlyLogBuffer struct with initialized fields
         EarlyLogBuffer {
             size,
             index,
             last_flush_index,
             buffer,
         }
     }
 }
 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct Framebuffer {
     pub enabled: bool,
     pub addr: usize,
     pub width: u32,
     pub height: u32,
     pub pitch: u32,
     pub depth: u32,
     pub size: u64,
 }
 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct Console {
     pub cursor_pos: u32,
     pub line: u32,
     pub max_chars: u32,
     pub max_line: u32,
 }
 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct Serial {
     pub enabled: bool,
     pub port: u16,
 }
 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct ComponentInfo {
     pub present: bool,
     pub address: usize,
     pub size: usize,
     pub state: u8,
 }
 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct CriticalComponents {
     pub vfs: ComponentInfo,
     pub mm: ComponentInfo,
     pub pm: ComponentInfo,
     pub sched: ComponentInfo,
     pub disk_driver: ComponentInfo,
     pub fb: ComponentInfo,
     pub disk_dev: ComponentInfo,
     pub tty_dev: ComponentInfo,
 }
 

 #[derive(Copy, Clone)]
 #[repr(C)]
 pub struct MemoryInfo {
     pub total_physical_memory: usize,
     pub available_memory: usize,
     pub memory_map: [MemoryMapEntry; 100],
     pub memory_map_entries: u16,
 }

 impl Default for MemoryInfo {
    fn default() -> Self {
        // Initialize each entry in the array to its default value
        let memory_map = [MemoryMapEntry::default(); 100];

        // Construct MemoryMap struct with initialized entry array
        MemoryInfo { 
            total_physical_memory: 0,
            available_memory: 0,
            memory_map,
            memory_map_entries: 0,
        }
    }
}

 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct MemoryMapEntry {
     pub base_address: usize,
     pub length: usize,
     pub type_: u8,
 }
 
 #[derive(Default, Copy, Clone)]
 #[repr(C)]
 pub struct CPUInfo {
     pub clock_speed: u8,
     pub logical_cpus: u8,
 }
 
 #[derive(Default, Copy, Clone)]
 pub struct Config {
     pub headless: bool,
 }
 




/*
*
* i686 BootInfo table
*
*/
pub mod i686 {
    #[derive(Default)]
    #[repr(C)]
    pub struct ArchBootInfo {
        pub global_descriptor_table: GlobalDescriptorTable,
        pub gdt_pointer: GDTPointer,
    }

    
    #[repr(C, packed)]
    pub struct GDTEntry {
        limit_low:              u16,
        base_low:               u16,
        base_middle:            u8,
        access:                 u8,
        limit_high_granularity: u8,
        base_high:              u8,
    }


    impl GDTEntry {
        pub fn new(base: u32, limit: u32, access: u8) -> Self {
            GDTEntry {
                limit_low: (limit & 0xFFFF) as u16,
                base_low: (base & 0xFFFF) as u16,
                base_middle: ((base >> 16) & 0xFF) as u8,
                access,
                limit_high_granularity: (0xD << 4) | ((limit & 0x000F) as u8) as u8,     // Granularity is the same for every descriptor: 4KB page granularity, 32 bit protected mode
                base_high: ((base >> 24) & 0xFF) as u8,
            }
        }

        pub fn get_offset(&self, gdt: &GlobalDescriptorTable) -> u16 {
            // Calculate the offset of the given selector within the struct
            let base_ptr = gdt as *const GlobalDescriptorTable as usize;
            let selector_ptr = self as *const GDTEntry as usize;
            (selector_ptr - base_ptr) as u16
        }
    }


    #[repr(C, packed)]
    pub struct GlobalDescriptorTable {
        pub null:       u64,
        pub sys_code:   GDTEntry,
        pub sys_data:   GDTEntry,
        pub user_code:  GDTEntry,
        pub user_data:  GDTEntry,
    }


    impl GlobalDescriptorTable {
        pub fn get_selector_offset(&self, selector: &GDTEntry) -> u16 {
            // Calculate the offset of the given selector within the struct
            let base_ptr = self as *const GlobalDescriptorTable as usize;
            let selector_ptr = selector as *const GDTEntry as usize;
            (selector_ptr - base_ptr) as u16
        }
    }


    impl Default for GlobalDescriptorTable {
        
        fn default() -> Self {
            const RING0_CODE_SEG_ACCESS: u8 = 0x9A;
            const RING0_DATA_SEG_ACCESS: u8 = 0x96;
            const RING3_CODE_SEG_ACCESS: u8 = 0xFA;
            const RING3_DATA_SEG_ACCESS: u8 = 0xF6;

            Self {
                null:       0,                                                                // Null segment descriptor
                sys_code:   GDTEntry::new(0, 0xFFFFFF, RING0_CODE_SEG_ACCESS),                // Code segment descriptor (supervisor, ring 0)
                sys_data:   GDTEntry::new(0, 0xFFFFFF, RING0_DATA_SEG_ACCESS),                // Data segment descriptor (supervisor, ring 0)
                user_code:  GDTEntry::new(0, 0xFFFFFF, RING3_CODE_SEG_ACCESS),                // Code segment descriptor (usermode, ring 3)
                user_data:  GDTEntry::new(0, 0xFFFFFF, RING3_DATA_SEG_ACCESS),                // Data segment descriptor (usermode, ring 3)
            }

            // Code Segment Access Byte:
            //  --------------------
            // | 0 | _00 | 0 | 0000 |
            // | P | DPL | S | 1CRA |
            //  --------------------
            //
            // P: Present - Always set to 1 as these segments are always available for use
            // DPL: Descriptor Privilege Level - Which protection ring the descriptor runs as. (R0: 00, R3: 11)
            // S: Descriptor Type - Differentiates between system and our descriptors, always set to 1
            // CRA: Conforming, Readable, Accessed

            // Data Segment Access Byte:
            //  --------------------
            // | 0 | _00 | 0 | 0000 |
            // | P | DPL | S | 0EWA |
            //  --------------------
            //
            // P - Present:
            //      Always set to 1 as these segments are always available for use
            // DPL - Descriptor Privilege Level:
            //      Which protection ring the descriptor runs as. (R0: 00, R3: 11)
            // S - Descriptor Type:
            //      Differentiates between system descriptors and our descriptors, always set to 1
            // EWA - Expand, Writable, Accessed:
            //

            // Granularity byte
            //  -----------------------------
            // | 0 | 0 |   0   |  0  |  0000 |
            // | G | D | RSRVD | AVL |  LMIT |
            //  -----------------------------
            //
            // G - Granularity:
            //      0 = Limit field interpreted in bytes
            //      1 = Limit field interpreted in 4KB pages
            //
            // D - Default Operation Size:
            //      0 = 16 bit real mode
            //      1 = 32 bit protected mode
        }
    }


    /// Pointer to the Global Descriptor table
    #[derive(Default)]
    #[repr(C, packed)]
    pub struct GDTPointer {
        pub limit:  u16,
        pub base:   usize,
    }


    impl GDTPointer {
        pub fn new(base: usize) -> Self {
            GDTPointer {
                limit: (core::mem::size_of::<GlobalDescriptorTable>() as u16) - 1,
                base: base,
            }
        }
    }
}