//! Definitions from `<c3d/base.h>`

use crate::C3D_FixedAttribGetWritePtr;

#[inline]
pub unsafe fn C3D_FixedAttribSet(id: libc::c_int, x: f32, y: f32, z: f32, w: f32) {
    let ptr = C3D_FixedAttribGetWritePtr(id);
    (*ptr).c.copy_from_slice(&[x, y, z, w]);
}
