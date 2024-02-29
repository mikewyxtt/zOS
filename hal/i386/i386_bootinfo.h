#ifndef I386_BOOTINFO_H
#define I386_BOOTINFO_H

#include <stdint.h>

/* Boot information specific to i386 processors */
struct i386BootInfo {

    /* Contains the Global Descriptor Table */
    struct {
        struct {
            char Signature[8];
            uint8_t Checksum;
            char OEMID[6];
            uint8_t Revision;
            uint32_t RSDTAddress;
        } RSDP __attribute__ ((packed));
    } GDT;

    /* Contains the Interrupt Descriptor Table */
    struct {
        struct {
            char Signature[8];
            uint8_t Checksum;
            char OEMID[6];
            uint8_t Revision;
            uint32_t RSDTAddress;
        } RSDP __attribute__ ((packed));
    } IDT;
    /* ACPI Info */
    struct {
        struct {
            char Signature[8];
            uint8_t Checksum;
            char OEMID[6];
            uint8_t Revision;
            uint32_t RSDTAddress;
        } RSDP __attribute__ ((packed));
    } ACPI;
};

#endif
