#ifndef STRING_UTILS_H
#define STRING_UTILS_H
#include <stdint.h>
#include <stddef.h>

/* Calculates the length of a string */
int strlen(char* str);

/* Converts an integer to a string */
void itoa (char *buf, int base, int d);

/* Copy specified number of bytes from source address to destination address */
void memcpy(void *destination, void *source, size_t count);

/* Copy specified number of bytes from source address to destination address */
void memset(void *destination, uint8_t value, size_t count);

/* Copy specified number of bytes from source address to destination address,
    being mindful of overlapping source and destination values. */
void memmove(void *destination, void *source, size_t count);

#endif