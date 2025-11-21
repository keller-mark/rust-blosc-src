// Reference: https://github.com/gyscos/zstd-rs/blob/main/zstd-safe/zstd-sys/src/wasm_shim.rs
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::ffi::{c_void, c_char, c_int};
use std::ptr;

// Use 16 for alignment to be safe for SIMD/doubles (similar to max_align_t)
const ALIGN: usize = 16;
const HEADER_SIZE: usize = 16;

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_malloc(size: usize) -> *mut c_void {
    let layout = Layout::from_size_align_unchecked(size + HEADER_SIZE, ALIGN);
    let ptr = alloc(layout);
    if ptr.is_null() {
        return ptr::null_mut();
    }
    *(ptr as *mut usize) = size;
    ptr.add(HEADER_SIZE) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_calloc(nmemb: usize, size: usize) -> *mut c_void {
    let total_size = nmemb * size;
    let layout = Layout::from_size_align_unchecked(total_size + HEADER_SIZE, ALIGN);
    let ptr = alloc_zeroed(layout);
    if ptr.is_null() {
        return ptr::null_mut();
    }
    *(ptr as *mut usize) = total_size;
    ptr.add(HEADER_SIZE) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let real_ptr = (ptr as *mut u8).sub(HEADER_SIZE);
    let size = *(real_ptr as *mut usize);
    let layout = Layout::from_size_align_unchecked(size + HEADER_SIZE, ALIGN);
    dealloc(real_ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_getenv(_name: *const c_char) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_strtol(
    str: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> i32 {
    let mut current = str as *const u8;

    // Skip whitespace
    while *current == b' ' || (*current >= 9 && *current <= 13) {
        current = current.add(1);
    }

    let mut sign = 1;
    if *current == b'-' {
        sign = -1;
        current = current.add(1);
    } else if *current == b'+' {
        current = current.add(1);
    }

    let mut result: i64 = 0;
    let radix = if base == 0 { 10 } else { base as u32 };

    loop {
        let digit = match *current {
            d @ b'0'..=b'9' => (d - b'0') as u32,
            d @ b'a'..=b'z' => (d - b'a') as u32 + 10,
            d @ b'A'..=b'Z' => (d - b'A') as u32 + 10,
            _ => break,
        };

        if digit >= radix {
            break;
        }

        result = result * (radix as i64) + (digit as i64);
        current = current.add(1);
    }

    if !endptr.is_null() {
        *endptr = current as *mut c_char;
    }

    (result * sign) as i32
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_qsort(
    base: *mut c_void,
    nitems: usize,
    size: usize,
    compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
) {
    if nitems < 2 || size == 0 {
        return;
    }
    let compar = compar.expect("compar function cannot be null");

    // Insertion sort: simple, robust, and sufficient for small arrays (like Huffman tables)
    let base = base as *mut u8;
    let tmp = alloc(Layout::from_size_align_unchecked(size, 1));

    for i in 1..nitems {
        let mut j = i;
        while j > 0 {
            let ptr_j_minus_1 = base.add((j - 1) * size);
            let ptr_j = base.add(j * size);

            if compar(ptr_j_minus_1 as *const c_void, ptr_j as *const c_void) > 0 {
                // swap
                ptr::copy_nonoverlapping(ptr_j_minus_1, tmp, size);
                ptr::copy_nonoverlapping(ptr_j, ptr_j_minus_1, size);
                ptr::copy_nonoverlapping(tmp, ptr_j, size);
                j -= 1;
            } else {
                break;
            }
        }
    }
    dealloc(tmp, Layout::from_size_align_unchecked(size, 1));
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memcmp(
    s1: *const c_void,
    s2: *const c_void,
    n: usize,
) -> c_int {
    let s1 = std::slice::from_raw_parts(s1 as *const u8, n);
    let s2 = std::slice::from_raw_parts(s2 as *const u8, n);
    for (&a, &b) in s1.iter().zip(s2.iter()) {
        if a != b {
            return (a as c_int) - (b as c_int);
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memcpy(
    dest: *mut c_void,
    src: *const c_void,
    n: usize,
) -> *mut c_void {
    ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memmove(
    dest: *mut c_void,
    src: *const c_void,
    n: usize,
) -> *mut c_void {
    ptr::copy(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memset(
    dest: *mut c_void,
    c: c_int,
    n: usize,
) -> *mut c_void {
    ptr::write_bytes(dest as *mut u8, c as u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_strcat(
    dest: *mut c_char,
    src: *const c_char,
) -> *mut c_char {
    let mut d = dest;
    while *d != 0 {
        d = d.add(1);
    }
    let mut s = src;
    while *s != 0 {
        *d = *s;
        d = d.add(1);
        s = s.add(1);
    }
    *d = 0;
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_strdup(s: *const c_char) -> *mut c_char {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    // +1 for null terminator. Use our shim malloc to ensure free works correctly.
    let dest = rust_zstd_wasm_shim_malloc(len + 1) as *mut c_char;
    if dest.is_null() {
        return ptr::null_mut();
    }
    ptr::copy_nonoverlapping(s, dest, len + 1);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_strcmp(
    s1: *const c_char,
    s2: *const c_char,
) -> c_int {
    let mut s1 = s1 as *const u8;
    let mut s2 = s2 as *const u8;
    while *s1 != 0 && *s1 == *s2 {
        s1 = s1.add(1);
        s2 = s2.add(1);
    }
    (*s1 as c_int) - (*s2 as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn sprintf(
    dest: *mut c_char,
    format: *const c_char,
) -> c_int {
    // Simple implementation: copy format string to dest, ignoring format specifiers and arguments.
    // This avoids the need for variadic support in Rust, which is unstable/complex for WASM targets.
    let mut src = format as *const u8;
    let mut dst = dest as *mut u8;
    let mut len = 0;
    while *src != 0 {
        *dst = *src;
        src = src.add(1);
        dst = dst.add(1);
        len += 1;
    }
    *dst = 0;
    len as c_int
}
