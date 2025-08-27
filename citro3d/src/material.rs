#[derive(Debug, Default, Clone, Copy)]
pub struct Material {
    pub ambient: Option<Color>,
    pub diffuse: Option<Color>,
    pub specular0: Option<Color>,
    pub specular1: Option<Color>,
    pub emission: Option<Color>,
}
impl Material {
    pub fn to_raw(self) -> citro3d_sys::C3D_Material {
        citro3d_sys::C3D_Material {
            ambient: self.ambient.unwrap_or_default().to_parts_bgr(),
            diffuse: self.diffuse.unwrap_or_default().to_parts_bgr(),
            specular0: self.specular0.unwrap_or_default().to_parts_bgr(),
            specular1: self.specular1.unwrap_or_default().to_parts_bgr(),
            emission: self.emission.unwrap_or_default().to_parts_bgr(),
        }
    }
}

/// RGB color in linear space ([0, 1])
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
    /// Split into BGR ordered parts
    ///
    /// # Reason for existence
    /// The C version of [`Material`] expects colours in BGR order (don't ask why it is beyond my comprehension)
    /// so we have to reorder when converting
    pub fn to_parts_bgr(self) -> [f32; 3] {
        [self.b, self.g, self.r]
    }
}
