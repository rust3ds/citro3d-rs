use citro3d_sys::{
    C3D_RenderTargetCreate, C3D_RenderTargetDelete, C3D_DEPTHTYPE, GPU_COLORBUF, GPU_DEPTHBUF,
};
use ctru::gfx;
use ctru::services::gspgpu;

use crate::{Error, Result};

/// A render target for `citro3d`. This is the data structure which handles sending
/// data to the GPU
pub struct Target {
    tag: *mut citro3d_sys::C3D_RenderTarget_tag,
}

impl Target {
    /// Create a new render target with the specified size, color format,
    /// and depth format.
    ///
    /// # Errors
    ///
    /// Fails if the specified sizes are invalid, or the target could not be
    /// created.
    pub fn new(
        width: u32,
        height: u32,
        color_format: ColorFormat,
        depth_format: DepthFormat,
    ) -> Result<Self> {
        let tag = unsafe {
            C3D_RenderTargetCreate(
                width.try_into()?,
                height.try_into()?,
                color_format as GPU_COLORBUF,
                depth_format.as_raw(),
            )
        };

        if tag.is_null() {
            Err(Error::FailedToInitialize)
        } else {
            Ok(Self { tag })
        }
    }

    /// Sets the screen to actually display the output of this render target.
    pub fn set_output(&mut self, screen: &impl gfx::Screen, side: gfx::Side, transfer_flags: u32) {
        unsafe {
            citro3d_sys::C3D_RenderTargetSetOutput(
                self.tag,
                screen.as_raw(),
                side.into(),
                transfer_flags,
            );
        }
    }

    pub fn clear(&mut self, flags: ClearFlags, color: u32, depth: u32) {
        unsafe {
            citro3d_sys::C3D_RenderTargetClear(self.tag, flags.bits(), color, depth);
        }
    }

    // TODO: this should maybe be a method on C3DContext instead?
    pub fn set_for_draw(&mut self) {
        unsafe {
            citro3d_sys::C3D_FrameDrawOn(self.tag);
        }
    }
}

#[repr(u32)]
pub enum TransferFormat {
    RGBA8 = citro3d_sys::GX_TRANSFER_FMT_RGBA8,
    RGB8 = citro3d_sys::GX_TRANSFER_FMT_RGB8,
    RGB565 = citro3d_sys::GX_TRANSFER_FMT_RGB565,
    RGB5A1 = citro3d_sys::GX_TRANSFER_FMT_RGB5A1,
    RGBA4 = citro3d_sys::GX_TRANSFER_FMT_RGBA4,
}

// TODO: more flags
bitflags::bitflags! {
    pub struct TransferFlags: u32 {
        const SCALE_NO = citro3d_sys::GX_TRANSFER_SCALE_NO;
        const SCALE_X = citro3d_sys::GX_TRANSFER_SCALE_X;
        const SCALE_XY = citro3d_sys::GX_TRANSFER_SCALE_XY;
    }
}

bitflags::bitflags! {
    pub struct ClearFlags: u32 {
        const COLOR = citro3d_sys::C3D_CLEAR_COLOR;
        const DEPTH = citro3d_sys::C3D_CLEAR_DEPTH;
        const ALL = citro3d_sys::C3D_CLEAR_ALL;
    }
}

impl Drop for Target {
    fn drop(&mut self) {
        unsafe {
            C3D_RenderTargetDelete(self.tag);
        }
    }
}

/// The color format to use when rendering on the GPU.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum ColorFormat {
    /// 8-bit Red + 8-bit Green + 8-bit Blue + 8-bit Alpha.
    RGBA8 = citro3d_sys::GPU_RB_RGBA8,
    /// 8-bit Red + 8-bit Green + 8-bit Blue.
    RGB8 = citro3d_sys::GPU_RB_RGB8,
    /// 5-bit Red + 5-bit Green + 5-bit Blue + 1-bit Alpha.
    RGBA5551 = citro3d_sys::GPU_RB_RGBA5551,
    /// 5-bit Red + 6-bit Green + 5-bit Blue.
    RGB565 = citro3d_sys::GPU_RB_RGB565,
    /// 4-bit Red + 4-bit Green + 4-bit Blue + 4-bit Alpha.
    RGBA4 = citro3d_sys::GPU_RB_RGBA4,
}

impl From<gspgpu::FramebufferFormat> for ColorFormat {
    fn from(format: gspgpu::FramebufferFormat) -> Self {
        match format {
            gspgpu::FramebufferFormat::Rgba8 => Self::RGBA8,
            gspgpu::FramebufferFormat::Rgb565 => Self::RGB565,
            gspgpu::FramebufferFormat::Rgb5A1 => Self::RGBA5551,
            gspgpu::FramebufferFormat::Rgba4 => Self::RGBA4,
            fmt => panic!("Unsupported frame buffer format {fmt:?}"),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DepthFormat {
    Depth16 = citro3d_sys::GPU_RB_DEPTH16,
    Depth24 = citro3d_sys::GPU_RB_DEPTH24,
    Depth24Stencil8 = citro3d_sys::GPU_RB_DEPTH24_STENCIL8,
}

impl DepthFormat {
    fn as_raw(self) -> C3D_DEPTHTYPE {
        C3D_DEPTHTYPE {
            __e: self as GPU_DEPTHBUF,
        }
    }
}
