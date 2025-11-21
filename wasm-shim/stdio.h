#include <stddef.h>

#ifndef	_STDIO_H
#define	_STDIO_H	1

#define fprintf(expr, ...)
#define printf(...)
#define fflush(expr)

int sprintf(char *str, const char *format, ...);

#endif // _STDIO_H

