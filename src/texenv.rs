#![cfg(todo = "gpu_tev_macros")]
//! `<c3d/texenv.h>`

use super::*;
use libc::c_int;

pub unsafe fn C3D_TexEnvSrc(env: *mut C3D_TexEnv, mode: c_int, s1: c_int, s2: c_int, s3: c_int) {
    let param = gpu_tevsources!(s1, s2, s3);

    if mode & C3D_RGB as i32 != 0 {
        (*env).srcRgb = param as u16;
    }

    if mode & C3D_Alpha as i32 != 0 {
        (*env).srcAlpha = param as u16;
    }
}

pub unsafe fn C3D_TexEnvOp(env: *mut C3D_TexEnv, mode: c_int, o1: c_int, o2: c_int, o3: c_int) {
    let param = gpu_tevoperands!(o1, o2, o3);

    if mode & C3D_RGB as i32 != 0 {
        (*env).opRgb = param as u16;
    }

    if mode & C3D_Alpha as i32 != 0 {
        (*env).opAlpha = param as u16;
    }
}

pub unsafe fn C3D_TexEnvFunc(env: *mut C3D_TexEnv, mode: c_int, param: c_int) {
    if mode & C3D_RGB as i32 != 0 {
        (*env).funcRgb = param as u16;
    }

    if mode & C3D_Alpha as i32 != 0 {
        (*env).funcAlpha = param as u16;
    }
}
