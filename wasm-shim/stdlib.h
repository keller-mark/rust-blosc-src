#include <stddef.h>


#ifndef _STDLIB_H
#define _STDLIB_H 1

void* rust_zstd_wasm_shim_malloc(size_t size);
void* rust_zstd_wasm_shim_calloc(size_t nmemb, size_t size);
void rust_zstd_wasm_shim_free(void* ptr);
void rust_zstd_wasm_shim_qsort(void* base, size_t nitems, size_t size,
                               int (*compar)(const void*, const void*));

#define malloc(size) rust_zstd_wasm_shim_malloc(size)
#define calloc(nmemb, size) rust_zstd_wasm_shim_calloc(nmemb, size)
#define free(ptr) rust_zstd_wasm_shim_free(ptr)
#define qsort(base, nitems, size, compar) \
  rust_zstd_wasm_shim_qsort(base, nitems, size, compar)

#endif  // _STDLIB_H

#ifndef _BLOSC_STDLIB_H
#define _BLOSC_STDLIB_H	1

long int rust_zstd_wasm_shim_strtol(const char* str, char** endptr, int base);
char* rust_zstd_wasm_shim_getenv(const char* name);

void abort(void);

#define strtol(str, endptr, base) rust_zstd_wasm_shim_strtol(str, endptr, base)
#define getenv(name) rust_zstd_wasm_shim_getenv(name)

#endif  // _BLOSC_STDLIB_H
