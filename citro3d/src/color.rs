//! Color manipulation module.

/// RGB color in linear space ([0, 1]).
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Splits the color into RGB ordered parts.
    pub fn to_parts_rgb(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    /// Splits the color into BGR ordered parts.
    pub fn to_parts_bgr(self) -> [f32; 3] {
        [self.b, self.g, self.r]
    }
}
