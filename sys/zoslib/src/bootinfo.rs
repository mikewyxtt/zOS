pub struct MemoryMap {
    start:      usize,
    size:       usize,
    _type:      u8,
}

pub struct Extension {
    pub name:   [char; 24],
    pub addr:   usize,
    pub size:   usize,
}

#[repr(C)]
pub struct BootInfo<T> {
    pub magic:          u16,
    pub version:        [char; 8],
    pub size:           usize,

    // Framebuffer info
    pub fb_enabled:     bool,
    pub fb_addr:        usize,
    pub fb_width:       u32,
    pub fb_height:      u32,
    pub fb_pitch:       u32,
    pub fb_depth:       u32,
    pub fb_size:        u64,

    pub memory_map:     [MemoryMap; 24],
    pub extensions:     [Extension; 32],

    // Architecture specific stuff
    pub arch_info:      T,

    // Value that can be checked to ensure struct boundaries are correct
    pub end:            u16,
}