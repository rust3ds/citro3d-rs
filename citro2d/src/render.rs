//! Safe bindings to render 2d graphics to a [Target]
use std::cell::RefMut;

use ctru::services::gfx::Screen;

use crate::{Error, Result, shapes::Shape};

/// A color in RGBA format. The color is stored as a 32-bit integer
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub inner: u32,
}

impl Color {
    /// Create a new color with the given RGB values. Alpha is set to 255 (fully opaque).
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::new_with_alpha(r, g, b, 255)
    }

    /// Create a new color with the given RGBA values.
    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        let inner = r as u32 | (g as u32) << 8 | (b as u32) << 16 | (a as u32) << 24;
        Self { inner }
    }
}

impl Into<Color> for u32 {
    fn into(self) -> Color {
        Color { inner: self }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.inner
    }
}

/// HACK A 2D target, which technically is a 3D target, but we use it for 2D rendering.
/// There is a chance that this can be combined with the 3D target in the future.
#[doc(alias = "C3D_RenderTarget")]
pub struct Target<'screen> {
    pub raw: *mut citro2d_sys::C3D_RenderTarget_tag,
    // This is unused after construction, but ensures unique access to the
    // screen this target writes to during rendering
    _phantom_screen: RefMut<'screen, dyn Screen>,
}

impl<'screen> Target<'screen> {
    ///Creates a 2D [Target] for rendering. Even though it returns a C3D_RenderTarget_tag, it is required to use the C2D_CreateScreenTarget method
    pub fn new(screen: RefMut<'screen, dyn Screen>) -> Result<Self> {
        let raw =
            unsafe { citro2d_sys::C2D_CreateScreenTarget(screen.as_raw(), screen.side().into()) };

        if raw.is_null() {
            return Err(Error::FailedToInitialize);
        }

        Ok(Self {
            raw,
            _phantom_screen: screen,
        })
    }

    /// Clears the screen to a selected color
    pub fn clear(&mut self, color: Color) {
        unsafe {
            citro2d_sys::C2D_TargetClear(self.raw, color.inner);
        }
    }

    /// Renders a 2d shape to the [Target]
    pub fn render_2d_shape<S>(&self, shape: &S)
    where
        S: Shape,
    {
        shape.render();
    }
}
