#ifndef _BLOSC_UNISTD_H
#define _BLOSC_UNISTD_H	1

#include <stddef.h>
#include <stdint.h>

#define _SC_NPROCESSORS_ONLN 84

long sysconf(int name);

#endif /* _BLOSC_UNISTD_H */
