//! `<c3d/renderqueue.h>`

pub unsafe fn C3D_RenderTargetClear(
    target: *mut crate::C3D_RenderTarget,
    clearBits: crate::C3D_ClearBits,
    clearColor: u32,
    clearDepth: u32,
) {
    crate::C3D_FrameBufClear(&mut (*target).frameBuf, clearBits, clearColor, clearDepth);
}
