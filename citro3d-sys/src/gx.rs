//! Helper functions based on `<3ds/gpu/gx.h>`.

use crate::{GX_TRANSFER_FORMAT, GX_TRANSFER_SCALE};

#[inline]
pub fn GX_TRANSFER_FLIP_VERT(flip: bool) -> u32 {
    flip as u32
}

#[inline]
pub fn GX_TRANSFER_OUT_TILED(tiled: bool) -> u32 {
    (tiled as u32) << 1
}

#[inline]
pub fn GX_TRANSFER_RAW_COPY(raw_copy: bool) -> u32 {
    (raw_copy as u32) << 3
}

#[inline]
pub fn GX_TRANSFER_IN_FORMAT(format: GX_TRANSFER_FORMAT) -> u32 {
    format << 8
}

#[inline]
pub fn GX_TRANSFER_OUT_FORMAT(format: GX_TRANSFER_FORMAT) -> u32 {
    format << 12
}

#[inline]
pub fn GX_TRANSFER_SCALING(scale: GX_TRANSFER_SCALE) -> u32 {
    scale << 24
}
