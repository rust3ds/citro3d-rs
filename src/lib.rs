extern crate libc;
extern crate core;
#[macro_use] extern crate ctru_sys as libctru;

#[allow(warnings)]
mod bindgen;
pub mod base;
pub mod texenv;
pub mod uniforms;

pub use bindgen::*;
pub use base::*;
pub use texenv::*;
pub use uniforms::*;

#[link(name="citro3d")]
#[link(name="ctru")]
extern {}

impl Copy for C3D_FVec {}
impl Clone for C3D_FVec {
    fn clone(&self) -> Self {
        *self
    }
}

impl From<libctru::GPU_DEPTHBUF> for C3D_DEPTHTYPE {
    fn from(fmt: libctru::GPU_DEPTHBUF) -> Self {
        Self {
            __e: fmt,
        }
    }
}
