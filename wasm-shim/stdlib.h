#include <stddef.h>

#ifndef _BLOSC_STDLIB_H
#define _BLOSC_STDLIB_H	1

ong int rust_zstd_wasm_shim_strtol(const char* str, char** endptr, int base);
char* rust_zstd_wasm_shim_getenv(const char* name);

void abort(void);

#define strtol(str, endptr, base) rust_zstd_wasm_shim_strtol(str, endptr, base)
#define getenv(name) rust_zstd_wasm_shim_getenv(name)

#endif  // _BLOSC_STDLIB_H
