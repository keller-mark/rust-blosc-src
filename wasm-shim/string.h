#include <stdlib.h>

#ifndef	_STRING_H
#define	_STRING_H	1

int rust_zstd_wasm_shim_memcmp(const void *str1, const void *str2, size_t n);
void *rust_zstd_wasm_shim_memcpy(void *restrict dest, const void *restrict src, size_t n);
void *rust_zstd_wasm_shim_memmove(void *dest, const void *src, size_t n);
void *rust_zstd_wasm_shim_memset(void *dest, int c, size_t n);
char *rust_zstd_wasm_shim_strcat(char *dest, const char *src);
char *rust_zstd_wasm_shim_strdup(const char *s);
int rust_zstd_wasm_shim_strcmp(const char *s1, const char *s2);

inline int memcmp(const void *str1, const void *str2, size_t n) {
    return rust_zstd_wasm_shim_memcmp(str1, str2, n);
}

inline void *memcpy(void *restrict dest, const void *restrict src, size_t n) {
	return rust_zstd_wasm_shim_memcpy(dest, src, n);
}

inline void *memmove(void *dest, const void *src, size_t n) {
	return rust_zstd_wasm_shim_memmove(dest, src, n);
}

inline void *memset(void *dest, int c, size_t n) {
	return rust_zstd_wasm_shim_memset(dest, c, n);
}

inline char *strcat(char *dest, const char *src) {
	return rust_zstd_wasm_shim_strcat(dest, src);
}

inline char *strdup(const char *s) {
	return rust_zstd_wasm_shim_strdup(s);
}

inline int strcmp(const char *s1, const char *s2) {
	return rust_zstd_wasm_shim_strcmp(s1, s2);
}

#endif // _STRING_H
