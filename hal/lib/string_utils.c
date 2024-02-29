#include "string_utils.h"
#include <stddef.h>

int strlen(char* str) {
    int length = 0;

    while(*str++) {
        length++;
    }

    return length;
}


void itoa (char *buf, int base, int d)
{
  char *p = buf;
  char *p1, *p2;
  unsigned long ud = d;
  int divisor = 10;
  
  /*  If %d is specified and D is minus, put ‘-’ in the head. */
  if (base == 'd' && d < 0)
    {
      *p++ = '-';
      buf++;
      ud = -d;
    }
  else if (base == 'x')
    divisor = 16;

  /*  Divide UD by DIVISOR until UD == 0. */
  do
    {
      int remainder = ud % divisor;
      
      *p++ = (remainder < 10) ? remainder + '0' : remainder + 'a' - 10;
    }
  while (ud /= divisor);

  /*  Terminate BUF. */
  *p = 0;
  
  /*  Reverse BUF. */
  p1 = buf;
  p2 = p - 1;
  while (p1 < p2)
    {
      char tmp = *p1;
      *p1 = *p2;
      *p2 = tmp;
      p1++;
      p2--;
    }
}

void memcpy(void *destination, void *source, size_t count) {
    uint8_t *s = source;
    uint8_t *d = destination;

    for(size_t i = 0; i < count; ++i) {
      s[i] = d[i];
    }
}


void memset(void *destination, uint8_t value, size_t count) {
    uint8_t *d = destination;

    for(size_t i = 0; i < count; ++i) {
        d[i] = value;
    }
}


void memmove(void *destination, void *source, size_t count) {
    uint8_t *s = source;
    uint8_t *d = destination;

    if (s < d && s + count > d) {
        for (size_t i = count; i != 0; --i) {
          d[i - 1] = s[i - 1];
        }
    }
    else {
      for (size_t i = 0; i < count; ++i) {
        d[i] = s[i];
      }
    }
}