#include <stdlib.h>

#ifndef	_BLOSC_STRING_H
#define	_BLOSC_STRING_H	1

char *rust_zstd_wasm_shim_strcat(char *dest, const char *src);
char *rust_zstd_wasm_shim_strdup(const char *s);
int rust_zstd_wasm_shim_strcmp(const char *s1, const char *s2);

inline char *strcat(char *dest, const char *src) {
	return rust_zstd_wasm_shim_strcat(dest, src);
}

inline char *strdup(const char *s) {
	return rust_zstd_wasm_shim_strdup(s);
}

inline int strcmp(const char *s1, const char *s2) {
	return rust_zstd_wasm_shim_strcmp(s1, s2);
}

#endif // _BLOSC_STRING_H
