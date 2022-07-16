//! Safe Rust bindings to `citro3d`.

pub mod error;
pub mod render;
pub mod shader;
pub mod texture;
pub mod vbo;

use citro3d_sys::C3D_FrameDrawOn;
use ctru::gfx::Screen;
pub use error::{Error, Result};

/// The single instance for using `citro3d`. This is the base type that an application
/// should instantiate to use this library.
#[non_exhaustive]
#[derive(Debug)]
pub struct Instance;

impl Instance {
    /// Initialize the default `citro3d` instance.
    ///
    /// # Errors
    ///
    /// Fails if `citro3d` cannot be initialized.
    pub fn new() -> Result<Self> {
        Self::with_cmdbuf_size(citro3d_sys::C3D_DEFAULT_CMDBUF_SIZE)
    }

    /// Initialize the instance with a specified command buffer size.
    ///
    /// # Errors
    ///
    /// Fails if `citro3d` cannot be initialized.
    pub fn with_cmdbuf_size(size: u32) -> Result<Self> {
        if unsafe { citro3d_sys::C3D_Init(size) } {
            Ok(Self)
        } else {
            Err(Error::FailedToInitialize)
        }
    }

    /// Select the given render target for drawing the frame.
    ///
    /// # Errors
    ///
    /// Fails if the given target cannot be used for drawing.
    pub fn select_render_target<'s, S: Screen>(
        &mut self,
        target: &render::Target<'s, S>,
    ) -> Result<()> {
        let _ = self;
        if unsafe { C3D_FrameDrawOn(target.as_raw()) } {
            Ok(())
        } else {
            Err(Error::InvalidRenderTarget)
        }
    }

    /// Render a frame. The passed in function/closure can mutate the instance,
    /// such as to [select a render target](Self::select_render_target).
    pub fn render_frame_with(&mut self, f: impl FnOnce(&mut Self)) {
        unsafe {
            citro3d_sys::C3D_FrameBegin(
                // TODO: begin + end flags should be configurable
                citro3d_sys::C3D_FRAME_SYNCDRAW
                    .try_into()
                    .expect("const is valid u8"),
            );
        }

        f(self);

        unsafe {
            citro3d_sys::C3D_FrameEnd(0);
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            citro3d_sys::C3D_Fini();
        }
    }
}
