#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
#![feature(allocator_api)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]
#![doc(html_root_url = "https://rust3ds.github.io/citro3d-rs/crates")]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]

//! Safe Rust bindings to `citro3d`. This crate wraps `citro3d-sys` to provide
//! safer APIs for graphics programs targeting the 3DS.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]

pub mod attrib;
pub mod buffer;
pub mod color;
pub mod error;
pub mod fog;
pub mod light;
pub mod math;
pub mod render;
pub mod shader;
pub mod texenv;
pub mod texture;
pub mod uniform;

use std::cell::RefMut;
use std::fmt;
use std::rc::Rc;

use ctru::services::gfx::Screen;
pub use error::{Error, Result};

use crate::render::Frame;

pub mod macros {
    //! Helper macros for working with shaders.
    pub use citro3d_macros::*;
}

mod private {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
}

/// Representation of `citro3d`'s internal render queue. This is something that
/// lives in the global context, but it keeps references to resources that are
/// used for rendering, so it's useful for us to have something to represent its
/// lifetime.
struct RenderQueue;

/// The single instance for using `citro3d`. This is the base type that an application
/// should instantiate to use this library.
#[non_exhaustive]
#[must_use]
pub struct Instance {
    queue: Rc<RenderQueue>,
}

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instance").finish_non_exhaustive()
    }
}

impl Instance {
    /// Initialize the default `citro3d` instance.
    ///
    /// # Errors
    ///
    /// Fails if `citro3d` cannot be initialized.
    pub fn new() -> Result<Self> {
        Self::with_cmdbuf_size(citro3d_sys::C3D_DEFAULT_CMDBUF_SIZE.try_into().unwrap())
    }

    /// Initialize the instance with a specified command buffer size.
    ///
    /// # Errors
    ///
    /// Fails if `citro3d` cannot be initialized.
    #[doc(alias = "C3D_Init")]
    pub fn with_cmdbuf_size(size: usize) -> Result<Self> {
        if unsafe { citro3d_sys::C3D_Init(size) } {
            Ok(Self {
                queue: Rc::new(RenderQueue),
            })
        } else {
            Err(Error::FailedToInitialize)
        }
    }

    /// Create a new render target with the specified size, color format,
    /// and depth format.
    ///
    /// # Errors
    ///
    /// Fails if the target could not be created with the given parameters.
    #[doc(alias = "C3D_RenderTargetCreate")]
    #[doc(alias = "C3D_RenderTargetSetOutput")]
    pub fn render_target<'screen>(
        &self,
        width: usize,
        height: usize,
        screen: RefMut<'screen, dyn Screen>,
        depth_format: Option<render::DepthFormat>,
    ) -> Result<render::Target<'screen>> {
        render::Target::new(width, height, screen, depth_format, Rc::clone(&self.queue))
    }

    /// Render a frame.
    ///
    /// The passed in function/closure can access a [`Frame`] to emit draw calls.
    #[doc(alias = "C3D_FrameBegin")]
    #[doc(alias = "C3D_FrameEnd")]
    pub fn render_frame_with<'istance: 'frame, 'frame>(
        &'istance mut self,
        f: impl FnOnce(Frame<'frame>) -> Frame<'frame>,
    ) {
        let frame = f(Frame::new(self));

        // Explicit drop for FrameEnd (when the GPU command buffer is flushed).
        drop(frame);
    }
}

// This only exists to be an alias, which admittedly is kinda silly. The default
// impl should be equivalent though, since RenderQueue has a drop impl too.
impl Drop for Instance {
    #[doc(alias = "C3D_Fini")]
    fn drop(&mut self) {}
}

impl Drop for RenderQueue {
    fn drop(&mut self) {
        unsafe {
            citro3d_sys::C3D_Fini();
        }
    }
}

#[cfg(test)]
mod tests {
    use ctru::services::gfx::Gfx;

    use super::*;

    #[test]
    fn select_render_target() {
        let gfx = Gfx::new().unwrap();
        let screen = gfx.top_screen.borrow_mut();

        let mut instance = Instance::new().unwrap();
        let target = instance.render_target(10, 10, screen, None).unwrap();

        instance.render_frame_with(|mut frame| {
            frame.select_render_target(&target).unwrap();

            frame
        });

        // Check that we don't get a double-free or use-after-free by dropping
        // the global instance before dropping the target.
        drop(instance);
        drop(target);
    }
}
