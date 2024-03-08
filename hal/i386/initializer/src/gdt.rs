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
use chimera::hal::boot::bootinfo::i686::GDTPointer;

/// Assembly subroutine. Loads the GDT into gdtr and sets the segment registers
extern "C" { fn _load_gdt(gdt_pointer: &GDTPointer, code_segment_selector: u16, data_segment_selector: u16); }


/// Sets up and loads our GDT
pub fn setup_gdt(archbootinfo: &mut ArchBootInfo) {
    
    // Create the GDT pointer
    archbootinfo.gdt_pointer = GDTPointer::new(core::ptr::addr_of!(archbootinfo.global_descriptor_table) as usize);
    
    // Load the GDT
    let gdt = &archbootinfo.global_descriptor_table;
    unsafe { _load_gdt(&archbootinfo.gdt_pointer, gdt.sys_code.get_offset(&gdt), gdt.sys_data.get_offset(&gdt)); }
}
