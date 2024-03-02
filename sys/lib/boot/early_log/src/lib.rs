// lib.rs

#![no_std]

//pub mod boot {

    
#[derive(Default)]
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

#[derive(Default)]
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

#[derive(Default)]
#[repr(C)]
pub struct Console {
    pub cursor_pos: u32,
    pub line: u32,
    pub max_chars: u32,
    pub max_line: u32,
}

#[derive(Default)]
#[repr(C)]
pub struct Serial {
    pub enabled: bool,
    pub port: u16,
}

#[derive(Default)]
#[repr(C)]
pub struct ComponentInfo {
    pub present: bool,
    pub address: usize,
    pub size: usize,
    pub state: u8,
}

#[derive(Default)]
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

#[derive(Default)]
#[repr(C)]
pub struct MemoryInfo {
    pub total_physical_memory: usize,
    pub available_memory: usize,
    pub memory_map: MemoryMap,
}

#[repr(C)]
pub struct MemoryMap {
    pub entry: [MemoryMapEntry; 100],
}

impl Default for MemoryMap {
    fn default() -> Self {
        // Initialize each entry in the array to its default value
        let entry = [MemoryMapEntry::default(); 100];

        // Construct MemoryMap struct with initialized entry array
        MemoryMap { entry }
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct MemoryMapEntry {
    pub base_address: usize,
    pub length: usize,
    pub memory_type: u8,
}

#[derive(Default)]
#[repr(C)]
pub struct CPUInfo {
    pub clock_speed: u8,
    pub logical_cpus: u8,
}

#[derive(Default)]
#[repr(C)]
pub struct Config {
    pub headless: bool,
}
//}