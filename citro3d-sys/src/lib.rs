#![no_std]
#![allow(non_snake_case)]
#![allow(warnings)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(ambiguous_glob_reexports)]
#![allow(clippy::all)]
#![doc(html_root_url = "https://rust3ds.github.io/citro3d-rs/crates")]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]

// During testing, re-export types to trigger `ambiguous_glob_reexports`
// if we ended up regenerating the same type as upstream ctru-sys or libc.
#[cfg(test)]
pub use ctru_sys::*;
#[cfg(test)]
pub use libc::*;

mod bindings {
    use ctru_sys::*;
    use libc::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

pub mod gx;
pub use gx::*;

// Prevent linking errors from the standard `test` library when running `cargo 3ds test --lib`.
#[cfg(all(test, not(rust_analyzer)))]
extern crate shim_3ds;
