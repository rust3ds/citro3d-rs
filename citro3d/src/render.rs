//! This module provides render target types and options for controlling transfer
//! of data to the GPU, including the format of color and depth data to be rendered.

use std::cell::RefMut;
use std::rc::Rc;

use citro3d_sys::{
    C3D_DEPTHTYPE, C3D_RenderTarget, C3D_RenderTargetCreate, C3D_RenderTargetDelete,
};
use ctru::services::gfx::Screen;
use ctru::services::gspgpu::FramebufferFormat;
use ctru_sys::{GPU_COLORBUF, GPU_DEPTHBUF};

use crate::{Error, RenderQueue, Result};

pub mod effect;
mod transfer;

/// A render target for `citro3d`. Frame data will be written to this target
/// to be rendered on the GPU and displayed on the screen.
#[doc(alias = "C3D_RenderTarget")]
pub struct Target<'screen> {
    raw: *mut citro3d_sys::C3D_RenderTarget,
    // This is unused after construction, but ensures unique access to the
    // screen this target writes to during rendering
    _screen: RefMut<'screen, dyn Screen>,
    _queue: Rc<RenderQueue>,
}

impl Drop for Target<'_> {
    #[doc(alias = "C3D_RenderTargetDelete")]
    fn drop(&mut self) {
        unsafe {
            C3D_RenderTargetDelete(self.raw);
        }
    }
}

impl<'screen> Target<'screen> {
    /// Create a new render target with the given parameters. This takes a
    /// [`RenderQueue`] parameter to make sure this  [`Target`] doesn't outlive
    /// the render queue.
    pub(crate) fn new(
        width: usize,
        height: usize,
        screen: RefMut<'screen, dyn Screen>,
        depth_format: Option<DepthFormat>,
        queue: Rc<RenderQueue>,
    ) -> Result<Self> {
        let color_format: ColorFormat = screen.framebuffer_format().into();

        let raw = unsafe {
            C3D_RenderTargetCreate(
                width.try_into()?,
                height.try_into()?,
                color_format as GPU_COLORBUF,
                depth_format.map_or(C3D_DEPTHTYPE { __i: -1 }, DepthFormat::as_raw),
            )
        };

        if raw.is_null() {
            return Err(Error::FailedToInitialize);
        }

        // Set the render target to actually output to the given screen
        let flags = transfer::Flags::default()
            .in_format(color_format.into())
            .out_format(color_format.into());

        unsafe {
            citro3d_sys::C3D_RenderTargetSetOutput(
                raw,
                screen.as_raw(),
                screen.side().into(),
                flags.bits(),
            );
        }

        Ok(Self {
            raw,
            _screen: screen,
            _queue: queue,
        })
    }

    /// Clear the render target with the given 32-bit RGBA color and depth buffer value.
    /// Use `flags` to specify whether color and/or depth should be overwritten.
    #[doc(alias = "C3D_RenderTargetClear")]
    pub fn clear(&mut self, flags: ClearFlags, rgba_color: u32, depth: u32) {
        unsafe {
            citro3d_sys::C3D_RenderTargetClear(self.raw, flags.bits(), rgba_color, depth);
        }
    }

    /// Return the underlying `citro3d` render target for this target.
    pub(crate) fn as_raw(&self) -> *mut C3D_RenderTarget {
        self.raw
    }
}

bitflags::bitflags! {
    /// Indicate whether color, depth buffer, or both values should be cleared.
    #[doc(alias = "C3D_ClearBits")]
    pub struct ClearFlags: u8 {
        /// Clear the color of the render target.
        const COLOR = citro3d_sys::C3D_CLEAR_COLOR;
        /// Clear the depth buffer value of the render target.
        const DEPTH = citro3d_sys::C3D_CLEAR_DEPTH;
        /// Clear both color and depth buffer values of the render target.
        const ALL = citro3d_sys::C3D_CLEAR_ALL;
    }
}

/// The color format to use when rendering on the GPU.
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[doc(alias = "GPU_COLORBUF")]
pub enum ColorFormat {
    /// 8-bit Red + 8-bit Green + 8-bit Blue + 8-bit Alpha.
    RGBA8 = ctru_sys::GPU_RB_RGBA8,
    /// 8-bit Red + 8-bit Green + 8-bit Blue.
    RGB8 = ctru_sys::GPU_RB_RGB8,
    /// 5-bit Red + 5-bit Green + 5-bit Blue + 1-bit Alpha.
    RGBA5551 = ctru_sys::GPU_RB_RGBA5551,
    /// 5-bit Red + 6-bit Green + 5-bit Blue.
    RGB565 = ctru_sys::GPU_RB_RGB565,
    /// 4-bit Red + 4-bit Green + 4-bit Blue + 4-bit Alpha.
    RGBA4 = ctru_sys::GPU_RB_RGBA4,
}

impl From<FramebufferFormat> for ColorFormat {
    fn from(format: FramebufferFormat) -> Self {
        match format {
            FramebufferFormat::Rgba8 => Self::RGBA8,
            FramebufferFormat::Rgb565 => Self::RGB565,
            FramebufferFormat::Rgb5A1 => Self::RGBA5551,
            FramebufferFormat::Rgba4 => Self::RGBA4,
            // this one seems unusual, but it appears to work fine:
            FramebufferFormat::Bgr8 => Self::RGB8,
        }
    }
}

/// The depth buffer format to use when rendering.
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[doc(alias = "GPU_DEPTHBUF")]
#[doc(alias = "C3D_DEPTHTYPE")]
pub enum DepthFormat {
    /// 16-bit depth.
    Depth16 = ctru_sys::GPU_RB_DEPTH16,
    /// 24-bit depth.
    Depth24 = ctru_sys::GPU_RB_DEPTH24,
    /// 24-bit depth + 8-bit Stencil.
    Depth24Stencil8 = ctru_sys::GPU_RB_DEPTH24_STENCIL8,
}

impl DepthFormat {
    fn as_raw(self) -> C3D_DEPTHTYPE {
        C3D_DEPTHTYPE {
            __e: self as GPU_DEPTHBUF,
        }
    }
}
