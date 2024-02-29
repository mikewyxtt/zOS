#include <stdint.h>
#include <stdbool.h>
#include "multiboot.h"
#include "../../lib/early_log.h"
#include "../../lib/string_utils.h"
#include "../../lib/bootinfo.h"
#include "../i386_bootinfo.h"
#include <cpuid.h>

/* Pointer to critical components tar archive */
static uintptr_t component_archive_address;

/* Size of the archive */
static size_t component_archive_size;

/* Initialize the BootInfo and i386BootInfo tables */
static void InitializeBootInfo(struct BootInfo *bootinfo, struct i386BootInfo *i386bootinfo, uintptr_t multiboot_header_addr);

/* Parse the Multiboot 2 Header to setup BootInfo table */
static void ParseMultibootHeader(struct BootInfo *bootinfo, struct i386BootInfo *i386bootinfo, uintptr_t multiboot_header_addr);


void main(uint32_t magic, uint32_t multiboot_header_addr) {
    // Ensure we were loaded by a Multiboot 2 compliant bootloader
    if(magic != MULTIBOOT2_BOOTLOADER_MAGIC) {
        // do something? hang for now..
        while(1);
    }

    // Create and initialize BootInfo and i386BootInfo tables
    struct BootInfo bootinfo;
    struct i386BootInfo i386bootinfo;
    memset((uintptr_t*)&bootinfo, 0, sizeof(bootinfo));
    memset((uintptr_t*)&i386bootinfo, 0, sizeof(i386bootinfo));
    InitializeBootInfo(&bootinfo, &i386bootinfo, multiboot_header_addr);

    // Log system information
    early_log(&bootinfo, "Multiboot 2 Info:\n");
    early_log(&bootinfo, "\tMagic Number: 0x%x\n", magic);
    early_log(&bootinfo, "\tMultiboot header addr: 0x%x\n", multiboot_header_addr);

    early_log(&bootinfo, "Framebuffer Info:\n");
    early_log(&bootinfo, "\tEnabled: %d\n", bootinfo.Framebuffer.enabled);
    early_log(&bootinfo, "\tAddress: 0x%x\n", bootinfo.Framebuffer.addr);
    early_log(&bootinfo, "\tResolution: %dx%d\n", bootinfo.Framebuffer.width, bootinfo.Framebuffer.height);
    early_log(&bootinfo, "\tPitch: %d bytes\n", bootinfo.Framebuffer.pitch);
    early_log(&bootinfo, "\tDepth: %d bits\n", bootinfo.Framebuffer.depth * 8);
    early_log(&bootinfo, "\tSize: %d bytes\n", bootinfo.Framebuffer.size);

    early_log(&bootinfo, "Console Info:\n");
    early_log(&bootinfo, "\tMax chars: %d\n", bootinfo.Console.max_chars);
    early_log(&bootinfo, "\tMax lines: %d\n", bootinfo.Console.max_line);
    early_log(&bootinfo, "\tCursor position: %d\n", bootinfo.Console.cursor_pos);
    early_log(&bootinfo, "\tCursor line: %d\n", bootinfo.Console.line);

    early_log(&bootinfo, "Serial Port Info:\n");
    early_log(&bootinfo, "\tEnabled: %d\n", bootinfo.Serial.enabled);
    early_log(&bootinfo, "\tUsing Port: 0x%x\n", bootinfo.Serial.port);

    early_log(&bootinfo, "Memory Info:\n");
    early_log(&bootinfo, "\tTotal RAM (KB): %d\n", bootinfo.MemoryInfo.TotalPhysicalMemory);
    early_log(&bootinfo, "\tUsable RAM (KB): %d\n", bootinfo.MemoryInfo.AvailableMemory);
    early_log(&bootinfo, "\tReserved RAM (KB): %d\n", (bootinfo.MemoryInfo.TotalPhysicalMemory - bootinfo.MemoryInfo.AvailableMemory));

    for (int i = 0; bootinfo.MemoryInfo.MemoryMap.Entry[i].Length > 0; i++) {
        early_log(&bootinfo, "\tMemory Map Entry %d:\n", i);
        early_log(&bootinfo, "\t\t Base Address: 0x%x\n", bootinfo.MemoryInfo.MemoryMap.Entry[i].BaseAddress);
        early_log(&bootinfo, "\t\t Length: %d KB\n", bootinfo.MemoryInfo.MemoryMap.Entry[i].Length + 1);
        if (bootinfo.MemoryInfo.MemoryMap.Entry[i].Type == 0) {
            early_log(&bootinfo, "\t\t Type: Available\n");
        }
        else {
            early_log(&bootinfo, "\t\t Type: Reserved\n");
        }
    }

    early_log(&bootinfo, "ACPI Info:\n");
    early_log(&bootinfo, "\tSignature: %d\n", i386bootinfo.ACPI.RSDP.Signature);
    early_log(&bootinfo, "\tChecksum: %d\n", i386bootinfo.ACPI.RSDP.Checksum);
    early_log(&bootinfo, "\tVendor: %s\n", i386bootinfo.ACPI.RSDP.OEMID);
    early_log(&bootinfo, "\tRevision: %d\n", i386bootinfo.ACPI.RSDP.Revision);
    early_log(&bootinfo, "\tRSDT Address: 0x%x\n", i386bootinfo.ACPI.RSDP.RSDTAddress);
    
    early_log(&bootinfo, "Misc. Info:\n");
    early_log(&bootinfo, "\tLog buffer size: %d\n", bootinfo.EarlyLogBuffer.size);
    early_log(&bootinfo, "\tBoot paramaters: %s\n", bootinfo.params);

    uint32_t a,b,c,unused;
    __cpuid(0,unused,a,c,b);
    early_log(&bootinfo, "CPUID:\n");
    early_log(&bootinfo, "\tVendor Name: %.4s%.4s%.4s\n",(char*)&a, (char*)&b, (char*)&c);

    while(1);
}


static void InitializeBootInfo(struct BootInfo *bootinfo, struct i386BootInfo *i386bootinfo, uintptr_t multiboot_header_addr) {
    // Set default values (NOTE: All values are set to 0 up until this point.)

    // Early Log Buffer
    bootinfo->EarlyLogBuffer.size = sizeof(bootinfo->EarlyLogBuffer.buffer);

    // Serial Logging
    #ifdef SERIAL_LOG
        bootinfo->Serial.enabled = true;
        bootinfo->Serial.port = 0x3f8;
    #endif

    // Required components list
    //bootinfo->CriticalServers.MM.

    // Parse Multiboot 2 header to fill in the BootInfo table
    ParseMultibootHeader(bootinfo, i386bootinfo, multiboot_header_addr);
}


static void ParseMultibootHeader(struct BootInfo *bootinfo, struct i386BootInfo *i386bootinfo, uintptr_t multiboot_header_addr) {
    /* We parse the header twice. First we get the boot args and list of components that will be loaded */
    struct multiboot_tag *tag;

    for (tag = (struct multiboot_tag *) (multiboot_header_addr + 8);
          tag->type != MULTIBOOT_TAG_TYPE_END;
          tag = (struct multiboot_tag *) ((multiboot_uint8_t *) tag + ((tag->size + 7) & ~7))) {
        

        switch (tag->type) {

        // Retrieve the framebuffer information
        case MULTIBOOT_TAG_TYPE_FRAMEBUFFER:
            ; // Clang errors out if we define the struct right after the case statement...
            
            struct multiboot_tag_framebuffer *fbtag = (struct multiboot_tag_framebuffer *) tag;

            if (fbtag->common.framebuffer_type == 1) {
                // Type of 1 means RGB, 2 means EGA text mode (unsupported), 0 means indexed color (unsupported)
                bootinfo->Framebuffer.enabled = true;
                bootinfo->Framebuffer.addr = fbtag->common.framebuffer_addr;
                bootinfo->Framebuffer.width = fbtag->common.framebuffer_width;
                bootinfo->Framebuffer.height = fbtag->common.framebuffer_height;
                bootinfo->Framebuffer.pitch = fbtag->common.framebuffer_pitch;
                bootinfo->Framebuffer.depth = fbtag->common.framebuffer_bpp / 8;
                bootinfo->Framebuffer.size = bootinfo->Framebuffer.width * bootinfo->Framebuffer.height * bootinfo->Framebuffer.depth;

                // Since we have a framebuffer, initialize the console.
                bootinfo->Console.cursor_pos = 0;
                bootinfo->Console.line = 0;
                bootinfo->Console.max_chars = (bootinfo->Framebuffer.width / 8);
                bootinfo->Console.max_line = (bootinfo->Framebuffer.height / 16);
            }
            break;

        // Parse boot command line args
        case MULTIBOOT_TAG_TYPE_CMDLINE:
            bootinfo->params = ((struct multiboot_tag_string*) tag)->string;
            break;

        // Retrieve memory map
        case MULTIBOOT_TAG_TYPE_MMAP:
            ; // avoid clang error
            multiboot_memory_map_t *mmap;
            
            int i = 0;
            for ( mmap = ((struct multiboot_tag_mmap *) tag)->entries;
                (multiboot_uint8_t *) mmap < (multiboot_uint8_t *) tag + tag->size;
                mmap = (multiboot_memory_map_t *)((unsigned long) mmap + ((struct multiboot_tag_mmap *) tag)->entry_size)) {
                
                // Calculate total physical memory
                bootinfo->MemoryInfo.TotalPhysicalMemory += ((mmap->len >> 32) / 1024);
                bootinfo->MemoryInfo.TotalPhysicalMemory += ((mmap->len & 0xffffffff) / 1024);

                // Calculate usable memory and create memory map entries.
                if (mmap->type == MULTIBOOT_MEMORY_AVAILABLE) {
                    bootinfo->MemoryInfo.AvailableMemory += ((mmap->len >> 32) / 1024);
                    bootinfo->MemoryInfo.AvailableMemory += ((mmap->len & 0xffffffff) / 1024);

                    bootinfo->MemoryInfo.MemoryMap.Entry[i].BaseAddress = mmap->addr;
                    bootinfo->MemoryInfo.MemoryMap.Entry[i].Length = (mmap->len / 1024);
                    bootinfo->MemoryInfo.MemoryMap.Entry[i].Type = 0;
                }
                // These memory regions are reserved.
                else {
                    bootinfo->MemoryInfo.MemoryMap.Entry[i].BaseAddress = mmap->addr;
                    bootinfo->MemoryInfo.MemoryMap.Entry[i].Length = (mmap->len / 1024);
                    bootinfo->MemoryInfo.MemoryMap.Entry[i].Type = 1;
                }

                i++;
                }
            break;
        
        // Find critical components. GRUB unpacks the archive for us.
        case MULTIBOOT_TAG_TYPE_MODULE:
            ; // avoid the clang error...
            component_archive_address = ((struct multiboot_tag_module *) tag)->mod_start;
            component_archive_size = ((struct multiboot_tag_module *) tag)->mod_end - ((struct multiboot_tag_module *) tag)->mod_start;

        // Retrieve ACPI Information
        case MULTIBOOT_TAG_TYPE_ACPI_NEW:
            early_log(bootinfo, "WARNING: New ACPI revision detected. This revision is untested.");
        case MULTIBOOT_TAG_TYPE_ACPI_OLD:
            ; // avoid the clang error...
            struct multiboot_tag_old_acpi *acpi_tag = (struct multiboot_tag_old_acpi *) tag;

            // Copy the RSDP values so we can find the other ACPI stuff
            memcpy(&i386bootinfo->ACPI.RSDP.Signature, &acpi_tag->RSDP.Signature, sizeof(i386bootinfo->ACPI.RSDP.Signature));
            i386bootinfo->ACPI.RSDP.Checksum = acpi_tag->RSDP.Checksum;
            memcpy(&i386bootinfo->ACPI.RSDP.OEMID, &acpi_tag->RSDP.OEMID, sizeof(i386bootinfo->ACPI.RSDP.OEMID));
            i386bootinfo->ACPI.RSDP.Revision = acpi_tag->RSDP.Revision;
            i386bootinfo->ACPI.RSDP.RSDTAddress = acpi_tag->RSDP.RSDTAddress;
        default:
            break;
        }
    }
}
