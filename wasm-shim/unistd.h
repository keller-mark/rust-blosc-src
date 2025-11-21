#ifndef _UNISTD_H
#define _UNISTD_H

#include <stddef.h>
#include <stdint.h>

#define _SC_NPROCESSORS_ONLN 84

long sysconf(int name);

#endif /* _UNISTD_H */
