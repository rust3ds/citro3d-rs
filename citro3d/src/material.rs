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
            ambient: self.ambient.unwrap_or_default().to_parts(),
            diffuse: self.diffuse.unwrap_or_default().to_parts(),
            specular0: self.specular0.unwrap_or_default().to_parts(),
            specular1: self.specular1.unwrap_or_default().to_parts(),
            emission: self.emission.unwrap_or_default().to_parts(),
        }
    }
}

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
    pub fn to_parts(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}
