//! This module provides render target types and options for controlling transfer
//! of data to the GPU, including the format of color and depth data to be rendered.

use std::cell::RefMut;

use citro3d_sys::{
    C3D_RenderTarget, C3D_RenderTargetCreate, C3D_RenderTargetDelete, C3D_DEPTHTYPE,
};
use ctru::gfx::Screen;
use ctru::services::gspgpu::FramebufferFormat;
use ctru_sys::{GPU_COLORBUF, GPU_DEPTHBUF};

use crate::{buffers, Error, Result};

mod transfer;

/// A render target for `citro3d`. Frame data will be written to this target
/// to be rendered on the GPU and displayed on the screen.
pub struct Target<'screen> {
    raw: *mut citro3d_sys::C3D_RenderTarget,
    // This is unused after construction, but ensures unique access to the
    // screen this target writes to during rendering
    _screen: RefMut<'screen, dyn Screen>,
}

impl Drop for Target<'_> {
    fn drop(&mut self) {
        unsafe {
            C3D_RenderTargetDelete(self.raw);
        }
    }
}

impl<'screen> Target<'screen> {
    /// Create a new render target with the specified size, color format,
    /// and depth format.
    ///
    /// # Errors
    ///
    /// Fails if the target could not be created.
    pub fn new(
        width: u16,
        height: u16,
        screen: RefMut<'screen, dyn Screen>,
        depth_format: Option<DepthFormat>,
    ) -> Result<Self> {
        let color_format: ColorFormat = screen.get_framebuffer_format().into();

        let raw = unsafe {
            C3D_RenderTargetCreate(
                width.into(),
                height.into(),
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
        })
    }

    /// Clear the render target with the given 32-bit RGBA color and depth buffer value.
    /// Use `flags` to specify whether color and/or depth should be overwritten.
    pub fn clear(&mut self, flags: ClearFlags, rgba_color: u32, depth: u32) {
        unsafe {
            citro3d_sys::C3D_RenderTargetClear(self.raw, flags.bits(), rgba_color, depth);
        }
    }

    /// Return the underlying `citro3d` render target for this target.
    pub(crate) fn as_raw(&self) -> *mut C3D_RenderTarget {
        self.raw
    }

    pub fn draw_arrays(&mut self, primitive: buffers::Primitive, index: buffers::Index) {
        unsafe {
            citro3d_sys::C3D_DrawArrays(
                primitive as ctru_sys::GPU_Primitive_t,
                index.as_raw(),
                index.size(),
            );
        }
    }
}

bitflags::bitflags! {
    /// Indicate whether color, depth buffer, or both values should be cleared.
    pub struct ClearFlags: u32 {
        const COLOR = citro3d_sys::C3D_CLEAR_COLOR;
        const DEPTH = citro3d_sys::C3D_CLEAR_DEPTH;
        const ALL = citro3d_sys::C3D_CLEAR_ALL;
    }
}

/// The color format to use when rendering on the GPU.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
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
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
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
