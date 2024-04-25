/*  hal/i386/initializer/src/multiboot2.rs - Multiboot 2.0 constants / structs
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


 /* More information regarding the Multiboot2 standard can be found here:
  * https://www.gnu.org/software/zOS/manual/multiboot2/multiboot.html
  */


#![allow(dead_code)]

use core::ffi::c_char;

// The magic field should contain this.
pub const MULTIBOOT2_HEADER_MAGIC: u32 = 0xe85250d6;

// This should be in %eax.
pub const MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36d76289;

// Alignment of multiboot modules.
pub const MULTIBOOT_MOD_ALIGN: u32 = 0x00001000;

// Alignment of the multiboot info structure.
pub const MULTIBOOT_INFO_ALIGN: u32 = 0x00000008;

// Flags set in the 'flags' member of the multiboot header.
pub const MULTIBOOT_TAG_ALIGN: u32 = 8;
pub const MULTIBOOT_TAG_TYPE_END: u32 = 0;
pub const MULTIBOOT_TAG_TYPE_CMDLINE: u32 = 1;
pub const MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME: u32 = 2;
pub const MULTIBOOT_TAG_TYPE_MODULE: u32 = 3;
pub const MULTIBOOT_TAG_TYPE_BASIC_MEMINFO: u32 = 4;
pub const MULTIBOOT_TAG_TYPE_BOOTDEV: u32 = 5;
pub const MULTIBOOT_TAG_TYPE_MMAP: u32 = 6;
pub const MULTIBOOT_TAG_TYPE_VBE: u32 = 7;
pub const MULTIBOOT_TAG_TYPE_FRAMEBUFFER: u32 = 8;
pub const MULTIBOOT_TAG_TYPE_ELF_SECTIONS: u32 = 9;
pub const MULTIBOOT_TAG_TYPE_APM: u32 = 10;
pub const MULTIBOOT_TAG_TYPE_EFI32: u32 = 11;
pub const MULTIBOOT_TAG_TYPE_EFI64: u32 = 12;
pub const MULTIBOOT_TAG_TYPE_SMBIOS: u32 = 13;
pub const MULTIBOOT_TAG_TYPE_ACPI_OLD: u32 = 14;
pub const MULTIBOOT_TAG_TYPE_ACPI_NEW: u32 = 15;
pub const MULTIBOOT_TAG_TYPE_NETWORK: u32 = 16;
pub const MULTIBOOT_TAG_TYPE_EFI_MMAP: u32 = 17;
pub const MULTIBOOT_TAG_TYPE_EFI_BS: u32 = 18;
pub const MULTIBOOT_TAG_TYPE_EFI32_IH: u32 = 19;
pub const MULTIBOOT_TAG_TYPE_EFI64_IH: u32 = 20;
pub const MULTIBOOT_TAG_TYPE_LOAD_BASE_ADDR: u32 = 21;

pub const MULTIBOOT_HEADER_TAG_END: u32 = 0;
pub const MULTIBOOT_HEADER_TAG_INFORMATION_REQUEST: u32 = 1;
pub const MULTIBOOT_HEADER_TAG_ADDRESS: u32 = 2;
pub const MULTIBOOT_HEADER_TAG_ENTRY_ADDRESS: u32 = 3;
pub const MULTIBOOT_HEADER_TAG_CONSOLE_FLAGS: u32 = 4;
pub const MULTIBOOT_HEADER_TAG_FRAMEBUFFER: u32 = 5;
pub const MULTIBOOT_HEADER_TAG_MODULE_ALIGN: u32 = 6;
pub const MULTIBOOT_HEADER_TAG_EFI_BS: u32 = 7;
pub const MULTIBOOT_HEADER_TAG_ENTRY_ADDRESS_EFI32: u32 = 8;
pub const MULTIBOOT_HEADER_TAG_ENTRY_ADDRESS_EFI64: u32 = 9;
pub const MULTIBOOT_HEADER_TAG_RELOCATABLE: u32 = 10;

pub const MULTIBOOT_ARCHITECTURE_I386: u32 = 0;
pub const MULTIBOOT_ARCHITECTURE_MIPS32: u32 = 4;
pub const MULTIBOOT_HEADER_TAG_OPTIONAL: u32 = 1;

pub const MULTIBOOT_LOAD_PREFERENCE_NONE: u32 = 0;
pub const MULTIBOOT_LOAD_PREFERENCE_LOW: u32 = 1;
pub const MULTIBOOT_LOAD_PREFERENCE_HIGH: u32 = 2;

pub const MULTIBOOT_CONSOLE_FLAGS_CONSOLE_REQUIRED: u32 = 1;
pub const MULTIBOOT_CONSOLE_FLAGS_EGA_TEXT_SUPPORTED: u32 = 2;

pub const MULTIBOOT_MEMORY_AVAILABLE: u32 = 1;
pub const MULTIBOOT_MEMORY_RESERVED: u32 = 2;
pub const MULTIBOOT_MEMORY_ACPI_RECLAIMABLE: u32 = 3;
pub const MULTIBOOT_MEMORY_NVS: u32 = 4;
pub const MULTIBOOT_MEMORY_BADRAM: u32 = 5;

// Data structuers
#[repr(C)]
pub struct MultibootHeader {
    // Must be MULTIBOOT_MAGIC - see above.
    pub magic: u32,
    // ISA
    pub architecture: u32,
    // Total header length.
    pub header_length: u32,
    // The above fields plus this one must equal 0 mod 2^32.
    pub checksum: u32,
}

#[repr(C)]
pub struct MultibootHeaderTag {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
}

#[repr(C)]
pub struct MultibootHeaderTagInformationRequest {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
    pub requests: [u32], // Flexible array member
}

#[repr(C)]
pub struct MultibootHeaderTagAddress {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
    pub header_addr: u32,
    pub load_addr: u32,
    pub load_end_addr: u32,
    pub bss_end_addr: u32,
}

#[repr(C)]
pub struct MultibootHeaderTagEntryAddress {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
    pub entry_addr: u32,
}

#[repr(C)]
pub struct MultibootHeaderTagConsoleFlags {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
    pub console_flags: u32,
}

#[repr(C)]
pub struct MultibootHeaderTagFramebuffer {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
pub struct MultibootHeaderTagModuleAlign {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
}

#[repr(C)]
pub struct MultibootHeaderTagRelocatable {
    pub type_: u16,
    pub flags: u16,
    pub size: u32,
    pub min_addr: u32,
    pub max_addr: u32,
    pub align: u32,
    pub preference: u32,
    }

#[repr(C)]
pub struct MultibootColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    }

#[repr(C)]
pub struct MultibootTag {
    pub type_: u32,
    pub size: u32,
}

#[repr(C)]
pub struct MultibootTagString {
    pub type_: u32,
    pub size: u32,
    pub string: [u8], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagModule {
    pub type_: u32,
    pub size: u32,
    pub mod_start: u32,
    pub mod_end: u32,
    pub cmdline: [u8], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagBasicMeminfo {
    pub type_: u32,
    pub size: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
}

#[repr(C)]
pub struct MultibootTagBootdev {
    pub type_: u32,
    pub size: u32,
    pub biosdev: u32,
    pub slice: u32,
    pub part: u32,
}

#[repr(C)]
pub struct MultibootMemoryMap {
    pub addr: u64,
    pub len: u64,
    pub type_: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct MultibootTagMmap {
    pub type_: u32,
    pub size: u32,
    pub entry_size: u32,
    pub entry_version: u32,
    pub entries: [MultibootMemoryMap], // Flexible array member
}

#[repr(C)]
pub struct MultibootVbeInfoBlock {
    pub external_specification: [u8; 512],
}

#[repr(C)]
pub struct MultibootVbeModeInfoBlock {
    pub external_specification: [u8; 256],
}

#[repr(C)]
pub struct MultibootTagVbe {
    pub type_: u32,
    pub size: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub vbe_control_info: MultibootVbeInfoBlock,
    pub vbe_mode_info: MultibootVbeModeInfoBlock,
}

#[repr(C)]
pub struct MultibootTagFramebufferCommon {
    pub type_: u32,
    pub size: u32,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub reserved: u16,
}

#[repr(C)]
pub struct MultibootTagFramebuffer {
    pub common: MultibootTagFramebufferCommon,
    pub framebuffer_palette_num_colors: u16,
    pub framebuffer_palette: [MultibootColor], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagElfSections {
    pub type_: u32,
    pub size: u32,
    pub num: u32,
    pub entsize: u32,
    pub shndx: u32,
    pub sections: [c_char], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagApm {
    pub type_: u32,
    pub size: u32,
    pub version: u16,
    pub cseg: u16,
    pub offset: u32,
    pub cseg_16: u16,
    pub dseg: u16,
    pub flags: u16,
    pub cseg_len: u16,
    pub cseg_16_len: u16,
    pub dseg_len: u16,
}

#[repr(C)]
pub struct MultibootTagEfi32 {
    pub type_: u32,
    pub size: u32,
    pub pointer: u32,
}

#[repr(C)]
pub struct MultibootTagEfi64 {
    pub type_: u32,
    pub size: u32,
    pub pointer: u64,
}

#[repr(C)]
pub struct MultibootTagSmbios {
    pub type_: u32,
    pub size: u32,
    pub major: u8,
    pub minor: u8,
    pub reserved: [u8; 6],
    pub tables: [u8], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagOldAcpi {
    pub type_: u32,
    pub size: u32,
    pub rsdp: Rsdp,
}

#[repr(C)]
pub struct Rsdp {
    pub signature: [c_char; 8],
    pub checksum: u8,
    pub oem_id: [c_char; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

#[repr(C)]
pub struct MultibootTagNewAcpi {
    pub type_: u32,
    pub size: u32,
    pub rsdp: [u8], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagNetwork {
    pub type_: u32,
    pub size: u32,
    pub dhcpack: [u8], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagEfiMmap {
    pub type_: u32,
    pub size: u32,
    pub descr_size: u32,
    pub descr_vers: u32,
    pub efi_mmap: [u8], // Flexible array member
}

#[repr(C)]
pub struct MultibootTagEfi32Ih {
    pub type_: u32,
    pub size: u32,
    pub pointer: u32,
}

#[repr(C)]
pub struct MultibootTagEfi64Ih {
    pub type_: u32,
    pub size: u32,
    pub pointer: u64,
}

#[repr(C)]
pub struct MultibootTagLoadBaseAddr {
    pub type_: u32,
    pub size: u32,
    pub load_base_addr: u32,
}
