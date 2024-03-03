;  hal/i386/initializer/src/multiboot2.rs - Multiboot 2.0 constants / structs
;
;  chimera  --  Advanced *NIX System
;  Copyright (C) 2024  Free Software Foundation, Inc.
;
;  chimera is free software: you can redistribute it and/or modify
;  it under the terms of the GNU General Public License as published by
;  the Free Software Foundation, either version 3 of the License, or
;  (at your option) any later version.
;
;  chimera is distributed in the hope that it will be useful,
;  but WITHOUT ANY WARRANTY; without even the implied warranty of
;  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;  GNU General Public License for more details.
;
;  You should have received a copy of the GNU General Public License
;  along with GRUB. If not, see <http://www.gnu.org/licenses/>.
;

; Multiboot2 compliant initializer entry stub


[BITS 32]
SECTION .multiboot
ALIGN 8
;; Multiboot2 Header
DD  0xE85250D6                           ; Magic Number
DD  0x00                                 ; Architecture (0 is 32 bit protected mode)
DD  0x10                                 ; Header length (16 bytes)
DD  -(0xE85250D6 + 0x00 + 0x10)          ; Checksum

;; Multiboot tags
;; Multiboot information request
DW  0x01                                ; Tell GRUB we want to request info
DW  0x00                                ; Flags
DD  0x20                                ; Size
DD  0x01, 0x03, 0x04, 0x06, 0x08        ; Multiboot information request:
                                        ;  1: boot command line, 3: module info, 4: basic memory information, 6: memory map 8: fb info
DD  0x07                                 ; Padding (Everything myst be 8 byte aligned)

;; Request a framebuffer
DW  0x05                                ; Tag type. The presence of this tag tells GRUB we have framebuffer support
DW  0x00                                ; Flags. Not documented, unused
DD  0x14                                ; Size of 20 bytes
DD  0x00                                ; Preferred width. 0 = no preference
DD  0x00                                ; Preferred height. 0 = no preference
DD  0x00                                ; Preferred pitch. 0 = no preference

; End of tags
DW  0x00
DD  0x08

SECTION .text

EXTERN main

GLOBAL _start
_start:
    CLI                                 ; Disable Interupts

;    MOV     ESP, stack_bottom + 0x10000  ; 64KB stack
    mov     esp,  0x0007FFFF 
    MOV     EBP, ESP

    push 0                              ; Reset EFLAGS register
    popf                                ; ^^

    PUSH EBX                            ; Pass multiboot header address to main()
    PUSH EAX                            ; Pass magic number to main()
    CALL    main

.loop:                                  ; Infinite loop
    HLT
    JMP     .loop

;SECTION .bss
;stack_bottom    RESB    0x10000          ; 64KB stack ( was 0x4000 for both)
