use std::iter::FromIterator;
use std::path::PathBuf;

use bindgen::callbacks::{DeriveTrait, ImplementsTrait, ParseCallbacks};
use bindgen::{Builder, RustTarget};

fn main() {
    let devkitpro = std::env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    let devkitarm = std::env::var("DEVKITARM").expect("DEVKITARM not set in environment");

    let include_path = PathBuf::from(devkitpro).join(PathBuf::from_iter(["libctru", "include"]));
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
        // TODO functions,types,vars
        .blocklist_type("u(8|16|32|64)")
        .opaque_type("(GPU|GFX)_.*")
        .opaque_type("float24Uniform_s")
        .blocklist_file(".*/3ds/.*[.]h")
        .allowlist_file(".*/c3d/.*[.]h")
        .allowlist_file(".*/tex3ds[.]h")
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
