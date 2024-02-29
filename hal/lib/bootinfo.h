#ifndef BOOTINFO_H
#define BOOTINFO_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>


struct DriverInfo {
    /* Name of the driver */
    char name[25];

    /* Type of driver */
    char type[25];

    /* Physical start address of the driver */
    uintptr_t addr;

    /* Size of the driver in bytes */
    uint32_t size;
};

struct BootInfo {
    /* Early log buffer information */
    struct {
        /* Size of the buffer*/
        uint16_t size;

        /* Current index of log buffer*/
        uint16_t index;

        /* Index of buffer when it was last flushed */
        uint16_t last_flush_index;

        /* The early log buffer */
        char buffer[6144];
    } EarlyLogBuffer;

    /* Framebuffer information */
    struct {
        /* Framebuffer enabled? */
        bool enabled;

        /* Framebuffer physical address */
        uintptr_t addr;

        /* Framebuffer width in pixels */
        uint16_t width;

        /* Framebuffer height in pixels */
        uint16_t height;

        /* Framebuffer pitch */
        uint16_t pitch;

        /* Framebuffer depth (Bytes per pixel) */
        uint8_t depth;

        /* Size of the framebuffer in bytes */
        uint64_t size;
    } Framebuffer;

    /* Console Information */
    struct {
        int cursor_pos;
        int line;
        int max_chars;
        int max_line;
    } Console;

    /* Serial debugging information */
    struct {
        /* Serial enabled? */
        bool enabled;

        /* Serial port to output to */
        uint16_t port;
    } Serial;

    /* Address and size of critical components */
    struct {
        /* Virtual File System Server */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } vfs;

        /* Memory Management Server */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } mm;

        /* Processs management server */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } pm;

        /* Scheduler (server) */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } sched;

        /* Disk driver */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } DiskDriver;

        /* Framebuffer driver */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } fb;

        /* Filesystem driver */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } disk_dev;

        /* TTY driver */
        struct {
            bool present;
            uintptr_t Address;
            size_t Size;
            uint8_t State;
        } tty_dev;

    } CriticalComponents;

    /* List of drivers */
    // struct {
    //   /* Array containing driver inforation */
    //   struct DriverInfo driverinfo[25];

    //   /* Number of drivers present */
    //   uint32_t Count;
    // } DriverList;

    struct {
        /* Total system memory in KB */
        size_t TotalPhysicalMemory;

        /* Usable memory in KB */
        size_t AvailableMemory;

        struct {
            struct {
                /* Base address of memory region */
                uintptr_t BaseAddress;

                /* Length of region in KB */
                size_t Length;

                /* Type of memory region. 0 = Available for use, 1 = Reserved */
                uint8_t Type;
            } Entry[100];
        } MemoryMap;
    } MemoryInfo;

    struct {
        uint8_t ClockSpeed;
        uint8_t logical_cpus;
    } CPUInfo;

    /* Boot parameters passed in from bootloader */
    char *params;
    struct {
        bool headless;
    } Config;
};

#endif