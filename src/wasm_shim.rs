// Reference: https://github.com/gyscos/zstd-rs/blob/main/zstd-safe/zstd-sys/src/wasm_shim.rs
use alloc::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use core::ffi::{c_void, c_char, c_int};
use core::ptr;

// Use 16 for alignment to be safe for SIMD/doubles (similar to max_align_t)
const USIZE_ALIGN: usize = core::mem::align_of::<usize>();
const USIZE_SIZE: usize = core::mem::size_of::<usize>();

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_qsort(
    base: *mut c_void,
    n_items: usize,
    size: usize,
    compar: extern "C" fn(*const c_void, *const c_void) -> c_int,
) {
    unsafe {
        match size {
            1 => qsort::<1>(base, n_items, compar),
            2 => qsort::<2>(base, n_items, compar),
            4 => qsort::<4>(base, n_items, compar),
            8 => qsort::<8>(base, n_items, compar),
            16 => qsort::<16>(base, n_items, compar),
            _ => panic!("Unsupported qsort item size"),
        }
    }
}

unsafe fn qsort<const N: usize>(
    base: *mut c_void,
    n_items: usize,
    compar: extern "C" fn(*const c_void, *const c_void) -> c_int,
) {
    let base: &mut [[u8; N]] =
        core::slice::from_raw_parts_mut(base as *mut [u8; N], n_items);
    base.sort_unstable_by(|a, b| {
        match compar(a.as_ptr() as *const c_void, b.as_ptr() as *const c_void)
        {
            ..=-1 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            1.. => core::cmp::Ordering::Greater,
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_malloc(size: usize) -> *mut c_void {
    wasm_shim_alloc::<false>(size)
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memcmp(
    str1: *const c_void,
    str2: *const c_void,
    n: usize,
) -> i32 {
    // Safety: function contracts requires str1 and str2 at least `n`-long.
    unsafe {
        let str1: &[u8] = core::slice::from_raw_parts(str1 as *const u8, n);
        let str2: &[u8] = core::slice::from_raw_parts(str2 as *const u8, n);
        match str1.cmp(str2) {
            core::cmp::Ordering::Less => -1,
            core::cmp::Ordering::Equal => 0,
            core::cmp::Ordering::Greater => 1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_calloc(
    nmemb: usize,
    size: usize,
) -> *mut c_void {
    // note: calloc expects the allocation to be zeroed
    wasm_shim_alloc::<true>(nmemb * size)
}

#[inline]
fn wasm_shim_alloc<const ZEROED: bool>(size: usize) -> *mut c_void {
    // in order to recover the size upon free, we store the size below the allocation
    // special alignment is never requested via the malloc API,
    // so it's not stored, and usize-alignment is used
    // memory layout: [size] [allocation]

    let full_alloc_size = size + USIZE_SIZE;

    unsafe {
        let layout =
            Layout::from_size_align_unchecked(full_alloc_size, USIZE_ALIGN);

        let ptr = if ZEROED {
            alloc_zeroed(layout)
        } else {
            alloc(layout)
        };

        // SAFETY: ptr is usize-aligned and we've allocated sufficient memory
        ptr.cast::<usize>().write(full_alloc_size);

        ptr.add(USIZE_SIZE).cast()
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_free(ptr: *mut c_void) {
    // the layout for the allocation needs to be recovered for dealloc
    // - the size must be recovered from directly below the allocation
    // - the alignment will always by USIZE_ALIGN

    let alloc_ptr = ptr.sub(USIZE_SIZE);
    // SAFETY: the allocation routines must uphold having a valid usize below the provided pointer
    let full_alloc_size = alloc_ptr.cast::<usize>().read();

    let layout =
        Layout::from_size_align_unchecked(full_alloc_size, USIZE_ALIGN);
    dealloc(alloc_ptr.cast(), layout);
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memcpy(
    dest: *mut c_void,
    src: *const c_void,
    n: usize,
) -> *mut c_void {
    core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memmove(
    dest: *mut c_void,
    src: *const c_void,
    n: usize,
) -> *mut c_void {
    core::ptr::copy(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn rust_zstd_wasm_shim_memset(
    dest: *mut c_void,
    c: c_int,
    n: usize,
) -> *mut c_void {
    core::ptr::write_bytes(dest as *mut u8, c as u8, n);
    dest
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
