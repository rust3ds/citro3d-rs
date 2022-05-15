//! Safe Rust bindings to `citro3d`.

pub mod error;
pub mod render;
pub mod shader;
pub mod texture;
pub mod vbo;

use ctru::gfx::RawFrameBuffer;
pub use error::{Error, Result};

use render::Target;

/// The base context for using `citro3d`. This type must be used for
#[non_exhaustive]
#[derive(Debug)]
pub struct C3DContext;

impl C3DContext {
    /// Initialize the default context.
    ///
    /// # Errors
    ///
    /// Fails if the `citro3d` library cannot be initialized.
    pub fn new() -> Result<Self> {
        Self::with_command_buffer_size(citro3d_sys::C3D_DEFAULT_CMDBUF_SIZE)
    }

    /// Initialize the context with a specified command buffer
    ///
    /// # Errors
    ///
    /// Fails if the `citro3d` library cannot be initialized.
    pub fn with_command_buffer_size(size: u32) -> Result<Self> {
        if unsafe { citro3d_sys::C3D_Init(size) } {
            Ok(Self)
        } else {
            Err(Error::FailedToInitialize)
        }
    }

    /// Create a default render target for the given screen.
    ///
    /// # Errors
    ///
    /// Fails if the render target could not be created.
    pub fn render_target_for_screen(
        &self,
        frame_buffer: &RawFrameBuffer,
        color_format: render::ColorFormat,
        depth_format: render::DepthFormat,
    ) -> Result<Target> {
        Target::new(
            frame_buffer.width.into(),
            frame_buffer.height.into(),
            color_format,
            depth_format,
        )
    }
}

impl Drop for C3DContext {
    fn drop(&mut self) {
        unsafe {
            citro3d_sys::C3D_Fini();
        }
    }
}
