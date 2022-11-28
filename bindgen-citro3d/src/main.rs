//! This is meant to be run as a "script" to generate bindings to `citro3d`.
//! We use this instead of `bindgen-cli` to enable the use of [`CustomCallbacks`]
//! with [`bindgen`] as a library for finer grained control of the bindings.

use std::iter::FromIterator;
use std::path::PathBuf;

use bindgen::callbacks::{DeriveTrait, ImplementsTrait, ParseCallbacks};
use bindgen::{Builder, RustTarget};

fn main() {
    let devkitpro = std::env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    let devkitarm = std::env::var("DEVKITARM").expect("DEVKITARM not set in environment");

    let include_path = PathBuf::from_iter([devkitpro.as_str(), "libctru", "include"]);
    let header = include_path.join("tex3ds.h");

    let sysroot = PathBuf::from(devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");

    let bindings = Builder::default()
        .header(header.to_str().unwrap())
        .rust_target(RustTarget::Nightly)
        .use_core()
        .trust_clang_mangling(false)
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .fit_macro_constants(true)
        .raw_line("use ctru_sys::*;")
        .must_use_type("Result")
        .blocklist_type("u(8|16|32|64)")
        .opaque_type("(GPU|GFX)_.*")
        .opaque_type("float24Uniform_s")
        .allowlist_file(".*/c3d/.*[.]h")
        .allowlist_file(".*/tex3ds[.]h")
        .blocklist_file(".*/3ds/.*[.]h")
        .blocklist_file(".*/sys/.*[.]h")
        .clang_args([
            "--target=arm-none-eabi",
            "--sysroot",
            sysroot.to_str().unwrap(),
            "-isystem",
            system_include.to_str().unwrap(),
            "-I",
            include_path.to_str().unwrap(),
            "-mfloat-abi=hard",
            "-march=armv6k",
            "-mtune=mpcore",
            "-mfpu=vfp",
            "-DARM11 ",
            "-D_3DS ",
            "-D__3DS__ ",
        ])
        .parse_callbacks(Box::new(CustomCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write(Box::new(std::io::stdout()))
        .expect("failed to write bindings");
}

/// Custom callback struct to allow us to mark some "known good types" as
/// [`Copy`], which in turn allows using Rust `union` instead of bindgen union
/// types. See
/// <https://rust-lang.github.io/rust-bindgen/using-unions.html#which-union-type-will-bindgen-generate>
/// for more info.
///
/// We do the same for [`Debug`] just for the convenience of derived Debug impls
/// on some `citro3d` types.
#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
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
