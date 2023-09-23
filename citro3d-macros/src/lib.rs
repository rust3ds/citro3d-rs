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
/// This is similar to the standard library's [`include_bytes!`] macro, for which
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
/// ```no_run
/// # use citro3d_macros::include_shader;
/// static SHADER_BYTES: &[u8] = include_shader!("assets/vshader.pica");
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

    let string_lit = match StringLit::try_from(&tokens[0]) {
        Ok(lit) => lit,
        Err(err) => return Ok(err.to_compile_error()),
    };

    // The cwd can change depending on whether this is running in a doctest or not:
    // https://users.rust-lang.org/t/which-directory-does-a-proc-macro-run-from/71917
    //
    // But the span's `source_file()` seems to always be relative to the cwd.
    let cwd = env::current_dir().expect("unable to determine working directory");
    let span = tokens[0].span();
    let invoking_source_file = span.source_file().path();
    let Some(invoking_source_dir) = invoking_source_file.parent() else {
        return Err("unable to find parent directory of invoking source file".into());
    };

    // By joining these three pieces, we arrive at approximately the same behavior as `include_bytes!`
    let shader_source_file = cwd.join(invoking_source_dir).join(string_lit.value());
    let shader_out_file: PathBuf = shader_source_file.with_extension("shbin");

    let out_dir = PathBuf::from(env!("OUT_DIR"));
    // This might be overkill, but it ensures we get a unique path if different
    // shaders with the same relative path are used within one program
    let relative_out_path = shader_out_file.canonicalize()?;

    let out_path = out_dir.join(
        relative_out_path
            .strip_prefix("/")
            .unwrap_or(&shader_out_file),
    );

    let parent_dir = out_path.parent().ok_or("invalid input filename")?;
    DirBuilder::new().recursive(true).create(parent_dir)?;

    let devkitpro = PathBuf::from(env!("DEVKITPRO"));

    let output = process::Command::new(devkitpro.join("tools/bin/picasso"))
        .arg("--out")
        .args([&out_path, &shader_source_file])
        .output()?;

    match output.status.code() {
        Some(0) => {}
        code => {
            let code = code.map_or_else(|| String::from("unknown"), |c| c.to_string());

            return Err(format!(
                "failed to compile shader: exit status from `picasso` was {code}: {}",
                String::from_utf8_lossy(&output.stderr),
            )
            .into());
        }
    }

    let bytes = std::fs::read(out_path)?;
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
