/*  hal/i386/initializer/src/start.S - Multiboot2 compliant initializer entry stub
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


.section .multiboot, "a"
.SET    MULTIBOOT2_HEADER_MAGIC,            0xE85250D6  // Magic Number
.SET    MULTIBOOT2_ARCHITECTURE,            0x00        // Architecture (0 is 32 bit protected mode)

/* Multiboot2 Header */
.ALIGN 8
multiboot_header:
    .LONG   MULTIBOOT2_HEADER_MAGIC                         
    .LONG   MULTIBOOT2_ARCHITECTURE
    .LONG   multiboot_header_end - multiboot_header
    .LONG   -(MULTIBOOT2_HEADER_MAGIC + MULTIBOOT2_ARCHITECTURE + (multiboot_header_end - multiboot_header))


    /* Multiboot information request */
    .WORD   0x01                                        // Tell GRUB we want to request info
    .WORD   0x00                                        // Flags
    .LONG   0x20                                        // Size
    .LONG   0x01, 0x03, 0x04, 0x06, 0x08                // Multiboot information request:
                                                        //  1: boot command line, 3: module info, 4: basic memory information, 6: memory map 8: fb info
    .LONG   0x00                                        // Padding (Everything myst be 8 byte aligned)

    // Request a framebuffer
    framebuffer_tag:
        .WORD   0x05                                    // Tag type. The presence of this tag tells GRUB we have framebuffer support
        .WORD   0x00                                    // Flags. Not documented, unused
        .LONG   (framebuffer_tag_end - framebuffer_tag) // Size of framebuffer tag
        .LONG   0x00                                    // Preferred width. 0 = no preference
        .LONG   0x00                                    // Preferred height. 0 = no preference
        .LONG   0x00                                    // Preferred pitch. 0 = no preference
    framebuffer_tag_end:

    // End of tags
    .WORD   0x00
    .LONG   0x08

multiboot_header_end:


.section .text

.EXTERN main

.GLOBAL _start
_start:
    CLI                                                 // Disable Interupts

//    MOV     ESP, stack_bottom + 0x10000               // 64KB stack

    MOVL    $0x0007FFFF, %ESP
    MOV     %ESP, %EBP

    PUSHL   $0                                          // Reset EFLAGS register
    POPFL                                               // ^^

    PUSHL   %EBX                                        // Pass multiboot header address to main()
    PUSHL   %EAX                                        // Pass magic number to main()
    CALL    main

.loop:                                                  // Infinite loop
    HLT
    JMP     .loop


//.SECTION .bss
//stack_bottom    RESB    0x10000                       // 64KB stack ( was 0x4000 for both)
