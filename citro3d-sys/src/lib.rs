#![no_std]
#![allow(non_snake_case)]
#![feature(untagged_unions)] // used for [`C3D_Mtx`]

pub mod base;
pub mod renderqueue;
pub mod texenv;
pub mod uniforms;

#[allow(warnings)]
#[allow(warnings)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(clippy::all)]
mod bindings;

pub use base::*;
pub use bindings::*;
pub use renderqueue::*;
pub use texenv::*;
pub use uniforms::*;
