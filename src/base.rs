#![allow(non_snake_case)]
// c3d/base.h

use libc::c_int;
use super::*;

pub unsafe fn C3D_FixedAttribSet(id: c_int, x: f32, y: f32, z: f32, w: f32) {
    let mut ptr = C3D_FixedAttribGetWritePtr(id);
    *(*ptr).c.as_mut() = [x, y, z, w];
}
