use cc::Build;
use std::env;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cc::Build::new();

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_mscv = env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    let add_file = |builder: &mut Build, folder: &str| {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "c" || extension == "cpp" || (!target_mscv && extension == "S") {
                    if  path.ends_with("c-blosc/blosc/shuffle-sse2.c") ||  path.ends_with("c-blosc/blosc/bitshuffle-sse2.c") {
                        if target_arch == "x86" || target_arch == "x86_64" {
                            // gate AVX2 files too, only on x86/x86_64
                            builder.file(path);
                        }
                    } else {
                        builder.file(path);
                    }
                
                }
            }
        }
    };

    add_file(&mut build, "c-blosc/blosc");
    let target_features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let target_features = target_features.split(',').collect::<Vec<_>>();

    /*
    build.define("SHUFFLE_SSE2_ENABLED", "0");
    build.define("SHUFFLE_AVX2_ENABLED", "0");
    build.flag_if_supported("-Os");
    build.flag_if_supported("-flto");
    build.flag_if_supported("-mno-avx");
    build.flag_if_supported("-mno-avx2");
    build.flag_if_supported("-mno-sse");
    build.flag_if_supported("-mno-sse2");
    */
    build.flag_if_supported("-Os");
    build.flag_if_supported("-flto");
    build.flag_if_supported("-mno-avx");
    build.flag_if_supported("-mno-avx2");
    build.flag_if_supported("-mno-sse");
    build.flag_if_supported("-mno-sse2");

    match target_arch.as_str() {
        // ----- x86 / x86_64: where AVX/SSE flags make sense -----
        "x86" | "x86_64" => {
            if target_mscv {
                // MSVC: don't enable /arch:AVX/AVX2; to avoid SSE2 on 32-bit use IA32.
                if target_arch == "x86" {
                    build.flag("/arch:IA32");
                }
                // On x86_64, SSE2 is baseline; nothing to add.
            } /*else {
                // GCC/Clang: explicitly disable AVX/AVX2
                build.flag_if_supported("-mno-avx");
                build.flag_if_supported("-mno-avx2");

                // SSE2 can only be disabled on 32-bit x86
                if target_arch == "x86" {
                    build.flag_if_supported("-mno-sse2");
                    build.flag_if_supported("-mno-sse");
                }
            }*/
        }

        // ----- everything else (wasm32, aarch64, arm, riscv, etc.) -----
        _ => {
            // Do NOT add x86-only flags; nothing to do for wasm32 here.
            // If you had accidentally added flags earlier, remove them.
        }
    }

    

    if cfg!(feature = "lz4") {
        let lz4_include_dir = std::env::var_os("DEP_LZ4_INCLUDE").unwrap();
        build.include(&lz4_include_dir);
        build.define("HAVE_LZ4", None);
    }
    if cfg!(feature = "zlib") {
        let zlib_include_dir = std::env::var_os("DEP_Z_INCLUDE").unwrap();
        build.include(&zlib_include_dir);
        build.define("HAVE_ZLIB", None);
    }

    if cfg!(feature = "zstd") {
        let zstd_include_dir = std::env::var_os("DEP_ZSTD_INCLUDE").unwrap();
        build.include(&zstd_include_dir);
        build.define("HAVE_ZSTD", None);
    }
    if cfg!(feature = "snappy") {
        let snappy_include_dir = std::env::var_os("DEP_SNAPPY_INCLUDE").unwrap();
        build.include(&snappy_include_dir);
        build.define("HAVE_SNAPPY", None);
    }

    let linklib = if cfg!(target_env = "msvc") {
        "libblosc"
    } else {
        "blosc"
    };
    build.compile(linklib);
}
