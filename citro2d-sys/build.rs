//! This build script generates bindings from `citro2d` on the fly at compilation
//! time into `OUT_DIR`, from which they can be included into `lib.rs`.

use std::env;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use bindgen::callbacks::{DeriveTrait, ImplementsTrait, ParseCallbacks};
use bindgen::{Builder, RustTarget};

fn main() {
    let devkitpro = env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");

    let devkitarm = std::env::var("DEVKITARM").expect("DEVKITARM not set in environment");
    println!("cargo:rerun-if-env-changed=DEVKITARM");

    let debug_symbols = env::var("DEBUG").unwrap();
    println!("cargo:rerun-if-env-changed=DEBUG");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-env-changed=OUT_DIR");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native={devkitpro}/libctru/lib");
    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols.as_str() {
            // Based on valid values described in
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            "0" | "false" | "none" => "citro2d",
            _ => "citro2dd",
        }
    );

    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols.as_str() {
            // Based on valid values described in
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            "0" | "false" | "none" => "citro3d",
            _ => "citro3dd",
        }
    );

    let include_path = PathBuf::from_iter([devkitpro.as_str(), "libctru", "include"]);
    let citro2d_h = include_path.join("citro2d.h");
    let three_ds_h = include_path.join("3ds.h");

    let sysroot = Path::new(devkitarm.as_str()).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let static_fns_path = Path::new("citro2d_statics_wrapper");

    let gcc_dir = PathBuf::from_iter([devkitarm.as_str(), "lib", "gcc", "arm-none-eabi"]);

    let gcc_include = gcc_dir
        .read_dir()
        .unwrap()
        // Assuming that there is only one gcc version of libs under the devkitARM dir
        .next()
        .unwrap()
        .unwrap()
        .path()
        .join("include");

    let bindings = Builder::default()
        .header(three_ds_h.to_str().unwrap())
        .header(citro2d_h.to_str().unwrap())
        .rust_target(RustTarget::Nightly)
        .use_core()
        .trust_clang_mangling(false)
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .fit_macro_constants(true)
        .raw_line("use ctru_sys::*;")
        .raw_line("use libc::FILE;")
        .must_use_type("Result")
        .blocklist_type("u(8|16|32|64)")
        .blocklist_type("FILE")
        .opaque_type("(GPU|GFX)_.*")
        .opaque_type("float24Uniform_s")
        .allowlist_file(".*/c2d/.*[.]h")
        .blocklist_file(".*/3ds/.*[.]h")
        .blocklist_file(".*/sys/.*[.]h")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join(static_fns_path))
        .clang_args([
            "--target=arm-none-eabi",
            "--sysroot",
            sysroot.to_str().unwrap(),
            "-isystem",
            system_include.to_str().unwrap(),
            "-isystem",
            gcc_include.to_str().unwrap(),
            "-I",
            include_path.to_str().unwrap(),
            "-mfloat-abi=hard",
            "-march=armv6k",
            "-mtune=mpcore",
            "-mfpu=vfp",
            "-DARM11 ",
            "-D_3DS ",
            "-D__3DS__ ",
            "-fshort-enums",
        ])
        .parse_callbacks(Box::new(CustomCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings");

    // Compile static inline fns wrapper
    let cc = Path::new(devkitarm.as_str()).join("bin/arm-none-eabi-gcc");
    let ar = Path::new(devkitarm.as_str()).join("bin/arm-none-eabi-ar");

    cc::Build::new()
        .compiler(cc)
        .archiver(ar)
        .include(&include_path)
        .file(out_dir.join(static_fns_path.with_extension("c")))
        .flag("-march=armv6k")
        .flag("-mtune=mpcore")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=vfp")
        .flag("-mtp=soft")
        .flag("-Wno-deprecated-declarations")
        .compile("citro2d_statics_wrapper");
}

/// Custom callback struct to allow us to mark some "known good types" as
/// [`Copy`], which in turn allows using Rust `union` instead of bindgen union types. See
/// <https://rust-lang.github.io/rust-bindgen/using-unions.html#which-union-type-will-bindgen-generate>
/// for more info.
///
/// We do the same for [`Debug`] just for the convenience of derived Debug impls
/// on some `citro2d` types.
///
/// Finally, we use [`doxygen_rs`] to transform the doc comments into something
/// easier to read in the generated documentation / hover documentation.
#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }

    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if let DeriveTrait::Copy | DeriveTrait::Debug = derive_trait {
            match name {
                "u64_" | "u32_" | "u16_" | "u8_" | "u64" | "u32" | "u16" | "u8" | "gfxScreen_t"
                | "gfx3dSide_t" => Some(ImplementsTrait::Yes),
                _ if name.starts_with("GPU_") => Some(ImplementsTrait::Yes),
                _ => None,
            }
        } else {
            None
        }
    }
}
