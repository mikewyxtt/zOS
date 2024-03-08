/*  hal/i386/initializer/src/gdt.rs - setup and initialize GDT
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

use chimera::hal::boot::bootinfo::i686::ArchBootInfo;
use chimera::hal::boot::bootinfo::i686::{GlobalDescriptorTable, GDTPointer};

extern "C" { fn _load_gdt(gdt_pointer: &GDTPointer, code_segment_selector: u16, data_segment_selector: u16); }


/// Sets up and loads our GDT
pub fn setup_gdt(archbootinfo: &mut ArchBootInfo) {

    // Find the segment selectors. These values are offsets (in bytes) to the start of each entry. Null selector is 0, sys_code is 0x8, sys_data is 0x10, etc.
    let code_segment_selector = core::ptr::addr_of!(archbootinfo.global_descriptor_table.sys_code) as usize - core::ptr::addr_of!(archbootinfo.global_descriptor_table) as usize;
    let data_segment_selector = core::ptr::addr_of!(archbootinfo.global_descriptor_table.sys_data) as usize - core::ptr::addr_of!(archbootinfo.global_descriptor_table) as usize;
    
    // Create the GDT pointer
    archbootinfo.gdt_pointer = GDTPointer {
        limit: ((core::ptr::addr_of!(archbootinfo.global_descriptor_table) as usize - core::mem::size_of::<GlobalDescriptorTable>() as usize) as u16) - 1,
        base: core::ptr::addr_of!(archbootinfo.global_descriptor_table) as usize,
    };

    // Load the GDT in
    unsafe { _load_gdt(&archbootinfo.gdt_pointer, code_segment_selector as u16, data_segment_selector as u16); }
}
