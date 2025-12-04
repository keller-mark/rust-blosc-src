//! This crate is for building `c-blosc` and linking to the static build.

#![allow(clippy::redundant_static_lifetimes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
mod wasm_shim;

include!("bindgen.rs");

#[cfg(feature = "zlib")]
extern crate libz_sys;

#[cfg(feature = "zstd")]
extern crate zstd_safe;

#[cfg(feature = "lz4")]
extern crate lz4_sys;

#[cfg(feature = "snappy")]
extern crate snappy_src;
