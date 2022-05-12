//! `<c3d/uniforms.h>`

use super::*;

use libc::c_int;

#[inline]
pub unsafe fn C3D_FVUnifWritePtr(type_: GPU_SHADER_TYPE, id: c_int, size: c_int) -> *mut C3D_FVec {
    for i in 0..size {
        C3D_FVUnifDirty[type_ as usize][(id + i) as usize] = true;
    }

    return &mut C3D_FVUnif[type_ as usize][id as usize];
}

#[inline]
pub unsafe fn C3D_FVUnifMtxNx4(type_: GPU_SHADER_TYPE, id: c_int, mtx: *const C3D_Mtx, num: c_int) {
    let ptr = C3D_FVUnifWritePtr(type_, id, num);

    for i in 0..num {
        *ptr.offset(i as isize) = (*mtx).r.as_ref()[i as usize];
    }
}

#[inline]
pub unsafe fn C3D_FVUnifMtx4x4(type_: GPU_SHADER_TYPE, id: c_int, mtx: *const C3D_Mtx) {
    C3D_FVUnifMtxNx4(type_, id, mtx, 4);
}

#[inline]
pub unsafe fn C3D_FVUnifSet(type_: GPU_SHADER_TYPE, id: c_int, x: f32, y: f32, z: f32, w: f32) {
    let ptr = C3D_FVUnifWritePtr(type_, id, 1);
    (*ptr).c.copy_from_slice(&[x, y, z, w]);
}
