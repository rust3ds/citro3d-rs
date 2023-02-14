//! Definitions from `<c3d/renderqueue.h>`

use crate::*;

#[inline]
pub unsafe fn C3D_RenderTargetDetachOutput(target: *mut C3D_RenderTarget) {
    C3D_RenderTargetSetOutput(core::ptr::null_mut(), (*target).screen, (*target).side, 0);
}

#[inline]
pub unsafe fn C3D_RenderTargetClear(
    target: *mut C3D_RenderTarget,
    clearBits: C3D_ClearBits,
    clearColor: u32,
    clearDepth: u32,
) {
    C3D_FrameBufClear(&mut (*target).frameBuf, clearBits, clearColor, clearDepth);
}
