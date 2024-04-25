/*  hal/i386/initializer/src/load_gdt.S - Assembly routine to load the gdt
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

.section .text


// fn _load_gdt(gdt_pointer: GDTPointer, code_segment_selector: u16, data_segment_selector: u16)

.globl _load_gdt
_load_gdt:
    cli                                     // Disable interrupts, just to be sure

    movl    4(%esp), %eax                   // gdt_pointer
    movl    8(%esp), %ebx                   // code_segment_selector
    movl    12(%esp), %ecx                  // data_segment_selector

    lgdt    (%eax)                          // Load the GDT

    movl    %ecx, %eax                      // Update the segment registers!
    movw    %ax, %ds                        // data segment
    movw    %ax, %ss                        // stack segment
    movw    %ax, %es                        // extra segment
    movw    %ax, %fs                        // extra segment
    movw    %ax, %gs                        // extra segment

    pushl   %ebx                            // We cannot set DS directly using mov, we must use a far jump. Normally you would do 'jmp Segment:Offset'
    push    $.flush_gdt                     //  but 'jmp %ebx:$.flush_gdt' is not valid syntax so we need to do the far jmp via the stack.
    ljmp    *(%esp)                         

.flush_gdt: 
    add     $8, %esp                        // We pushed 8 bytes onto the stack, we need to move the stack pointer back so ret can find the return address
    ret                                     // GDT loaded, return to caller
