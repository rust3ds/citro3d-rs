//! Procedural macro helpers for `citro3d`.

// we're already nightly-only so might as well use unstable proc macro APIs.
#![feature(proc_macro_span)]

use std::error::Error;
use std::fs::DirBuilder;
use std::path::PathBuf;
use std::{env, process};

use litrs::StringLit;
use proc_macro::TokenStream;
use quote::quote;

/// Compiles the given PICA200 shader using [`picasso`](https://github.com/devkitPro/picasso)
/// and returns the compiled bytes directly as a `&[u8]` slice.
///
/// This is similar to the standard library's [`include_bytes!`](std::include_bytes) macro, for which
/// file paths are relative to the source file where the macro is invoked.
///
/// The compiled shader binary will be saved in the caller's `$OUT_DIR`.
///
/// # Errors
///
/// This macro will fail to compile if the input is not a single string literal.
/// In other words, inputs like `concat!("foo", "/bar")` are not supported.
///
/// # Example
///
/// ```
/// use citro3d_macros::include_shader;
///
/// static SHADER_BYTES: &[u8] = include_shader!("../tests/integration.pica");
/// ```
///
/// # Errors
///
/// The macro will fail to compile if the `.pica` file cannot be found, or contains
/// `picasso` syntax errors.
///
/// ```compile_fail
/// # use citro3d_macros::include_shader;
/// static _ERROR: &[u8] = include_shader!("../tests/nonexistent.pica");
/// ```
///
/// ```compile_fail
/// # use citro3d_macros::include_shader;
/// static _ERROR: &[u8] = include_shader!("../tests/bad-shader.pica");
/// ```
#[proc_macro]
pub fn include_shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match include_shader_impl(input) {
        Ok(tokens) => tokens,
        Err(err) => {
            let err_str = err.to_string();
            quote! { compile_error!( #err_str ) }.into()
        }
    }
}

fn include_shader_impl(input: TokenStream) -> Result<TokenStream, Box<dyn Error>> {
    let tokens: Vec<_> = input.into_iter().collect();

    if tokens.len() != 1 {
        return Err(format!("expected exactly one input token, got {}", tokens.len()).into());
    }

    let shader_source_filename = &tokens[0];

    let string_lit = match StringLit::try_from(shader_source_filename) {
        Ok(lit) => lit,
        Err(err) => return Ok(err.to_compile_error()),
    };

    // The cwd can change depending on whether this is running in a doctest or not:
    // https://users.rust-lang.org/t/which-directory-does-a-proc-macro-run-from/71917
    //
    // But the span's `source_file()` seems to always be relative to the cwd.
    let cwd = env::current_dir()
        .map_err(|err| format!("unable to determine current directory: {err}"))?;

    let invoking_source_file = shader_source_filename
        .span()
        .local_file()
        .expect("source file not found");
    let Some(invoking_source_dir) = invoking_source_file.parent() else {
        return Ok(quote! {
            compile_error!(
                concat!(
                    "unable to find parent directory of current source file \"",
                    file!(),
                    "\""
                )
            )
        }
        .into());
    };

    // By joining these three pieces, we arrive at approximately the same behavior as `include_bytes!`
    let shader_source_file = cwd
        .join(invoking_source_dir)
        .join(string_lit.value())
        // This might be overkill, but it ensures we get a unique path if different
        // shaders with the same relative path are used within one program
        .canonicalize()
        .map_err(|err| format!("unable to resolve absolute path of shader source: {err}"))?;

    let shader_out_file: PathBuf = shader_source_file.with_extension("shbin");

    let out_dir = PathBuf::from(env!("OUT_DIR"));

    let out_path = out_dir.join(shader_out_file.components().skip(1).collect::<PathBuf>());
    // UNWRAP: we already canonicalized the source path, so it should have a parent.
    let out_parent = out_path.parent().unwrap();

    DirBuilder::new()
        .recursive(true)
        .create(out_parent)
        .map_err(|err| format!("unable to create output directory {out_parent:?}: {err}"))?;

    let devkitpro = PathBuf::from(env!("DEVKITPRO"));
    let picasso = devkitpro.join("tools/bin/picasso");

    let output = process::Command::new(&picasso)
        .arg("--out")
        .args([&out_path, &shader_source_file])
        .output()
        .map_err(|err| format!("unable to run {picasso:?}: {err}"))?;

    let error_code = match output.status.code() {
        Some(0) => None,
        code => Some(code.map_or_else(|| String::from("<unknown>"), |c| c.to_string())),
    };

    if let Some(code) = error_code {
        return Err(format!(
            "failed to compile shader: `picasso` exited with status {code}: {}",
            String::from_utf8_lossy(&output.stderr),
        )
        .into());
    }

    let bytes = std::fs::read(&out_path)
        .map_err(|err| format!("unable to read output file {out_path:?}: {err}"))?;

    let source_file_path = shader_source_file.to_string_lossy();

    let result = quote! {
        {
            // ensure the source is re-evaluted if the input file changes
            const _SOURCE: &[u8] = include_bytes! ( #source_file_path );

            // https://users.rust-lang.org/t/can-i-conveniently-compile-bytes-into-a-rust-program-with-a-specific-alignment/24049/2
            #[repr(C)]
            struct AlignedAsU32<Bytes: ?Sized> {
                _align: [u32; 0],
                bytes: Bytes,
            }

            // this assignment is made possible by CoerceUnsized
            const ALIGNED: &AlignedAsU32<[u8]> = &AlignedAsU32 {
                _align: [],
                // emits a token stream like `[10u8, 11u8, ... ]`
                bytes: [ #(#bytes),* ]
            };

            &ALIGNED.bytes
        }
    };

    Ok(result.into())
}
