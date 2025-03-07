extern crate cc;
extern crate glob;

use std::env;
use std::path::{Path, PathBuf};

macro_rules! build_clean {
    ($variant:expr) => {
        let internals_include_path = &std::env::var("DEP_PQCRYPTO_INTERNALS_INCLUDEPATH").unwrap();
        let common_dir = Path::new("pqclean/common");

        let mut builder = cc::Build::new();
        let target_dir: PathBuf = ["pqclean", "crypto_kem", $variant, "clean"]
            .iter()
            .collect();

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "wasi" {
            let wasi_sdk_path =
                &std::env::var("WASI_SDK_DIR").expect("missing environment variable: WASI_SDK_DIR");
            builder.flag(format!("--sysroot={}", wasi_sdk_path).as_str());
        }

        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();

        builder
            .include(internals_include_path)
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile(format!("{}_clean", $variant).as_str());
    };
}

macro_rules! build_avx2 {
    ($variant:expr) => {
        let internals_include_path = &std::env::var("DEP_PQCRYPTO_INTERNALS_INCLUDEPATH").unwrap();
        let common_dir = Path::new("pqclean/common");

        let mut builder = cc::Build::new();
        let target_dir: PathBuf = ["pqclean", "crypto_kem", $variant, "avx2"].iter().collect();

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "wasi" {
            let wasi_sdk_path =
                &std::env::var("WASI_SDK_DIR").expect("missing environment variable: WASI_SDK_DIR");
            builder.flag(format!("--sysroot={}", wasi_sdk_path).as_str());
        }

        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
        if target_env == "msvc" {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul");
        }

        builder
            .include(internals_include_path)
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile(format!("{}_avx2", $variant).as_str());
    };
}

fn main() {
    #[allow(unused_variables)]
    let aes_enabled = env::var("CARGO_FEATURE_AES").is_ok();
    #[allow(unused_variables)]
    let avx2_enabled = env::var("CARGO_FEATURE_AVX2").is_ok();
    #[allow(unused_variables)]
    let neon_enabled = env::var("CARGO_FEATURE_NEON").is_ok();
    #[allow(unused_variables)]
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    #[allow(unused_variables)]
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    #[allow(unused_variables)]
    let is_windows = target_os == "windows";
    #[allow(unused_variables)]
    let is_macos = target_os == "macos";

    build_clean!("mceliece348864");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece348864");
    }
    build_clean!("mceliece348864f");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece348864f");
    }
    build_clean!("mceliece460896");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece460896");
    }
    build_clean!("mceliece460896f");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece460896f");
    }
    build_clean!("mceliece6688128");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece6688128");
    }
    build_clean!("mceliece6688128f");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece6688128f");
    }
    build_clean!("mceliece6960119");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece6960119");
    }
    build_clean!("mceliece6960119f");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece6960119f");
    }
    build_clean!("mceliece8192128");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece8192128");
    }
    build_clean!("mceliece8192128f");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        build_avx2!("mceliece8192128f");
    }

    println!("cargo::rustc-check-cfg=cfg(enable_x86_avx2)");
    if target_arch == "x86_64" && avx2_enabled && !is_windows {
        // Print enableing flag for AVX2 implementation
        println!("cargo:rustc-cfg=enable_x86_avx2");
    }
}
