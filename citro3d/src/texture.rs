use std::mem::MaybeUninit;

use citro3d_sys::C3D_TexCalcMaxLevel;
pub use enums::*;

mod enums;

/// The maximum number of textures that can be bound at once
pub const TEXTURE_COUNT: usize = 4;
/// Minimum width and height of a texture
pub const MIN_TEX_SIZE: u16 = 8;
/// Maximum width and height of a texture
pub const MAX_TEX_SIZE: u16 = 1024;

/// Texture width and height must be between 8 and 1024 (inclusive)
#[derive(Debug, Clone)]
pub struct TextureParameters {
    pub width: u16,
    pub height: u16,
    pub max_level: u8,
    pub format: ColorFormat,
    pub mode: Mode,
    pub on_vram: bool,
}

/// Parameters used to initialize a `Texture`.
/// Pass it into `Texture::new` to create a new texture.
impl TextureParameters {
    /// `TextureParameters` to initialize a new 2D `Texture` with no mipmapping.
    pub const fn new_2d(width: u16, height: u16, format: ColorFormat) -> TextureParameters {
        TextureParameters {
            width,
            height,
            max_level: 0,
            format,
            mode: Mode::Tex2D,
            on_vram: false,
        }
    }

    /// `TextureParameters` to initialize a new 2D `Texture` with mipmapping.
    pub fn new_2d_with_mipmap(width: u16, height: u16, format: ColorFormat) -> TextureParameters {
        TextureParameters {
            width,
            height,
            max_level: unsafe { C3D_TexCalcMaxLevel(width as u32, height as u32) as u8 },
            format,
            mode: Mode::Tex2D,
            on_vram: false,
        }
    }

    /// `TextureParameters` to initialize a new 2D `Texture` with no mipmapping that is stored in VRAM.
    pub const fn new_2d_in_vram(width: u16, height: u16, format: ColorFormat) -> TextureParameters {
        TextureParameters {
            width,
            height,
            max_level: 0,
            format,
            mode: Mode::Tex2D,
            on_vram: true,
        }
    }

    /// `TextureParameters` to initialize a new 2D `Texture` for a shadow map.
    pub const fn new_shadow(width: u16, height: u16) -> TextureParameters {
        TextureParameters {
            width,
            height,
            max_level: 0,
            format: ColorFormat::Rgba8,
            mode: Mode::Shadow2D,
            on_vram: true,
        }
    }
}

impl Into<citro3d_sys::C3D_TexInitParams> for TextureParameters {
    fn into(self) -> citro3d_sys::C3D_TexInitParams {
        citro3d_sys::C3D_TexInitParams {
            width: self.width,
            height: self.height,
            _bitfield_align_1: [],
            _bitfield_1: citro3d_sys::C3D_TexInitParams::new_bitfield_1(
                self.max_level,
                self.format as u8,
                self.mode as u8,
                self.on_vram,
            ),
            __bindgen_padding_0: 0,
        }
    }
}

pub struct Texture {
    pub(crate) tex: citro3d_sys::C3D_Tex,
    pub(crate) format: ColorFormat,
    pub(crate) in_vram: bool,
}

impl Texture {
    /// Allocate a new texture with the given parameters.
    /// Texture allocation can fail if the texture size specified by the parameters is too small or
    /// large, or memory allocation fails.
    #[doc(alias = "C3D_TexInit")]
    pub fn new(params: TextureParameters) -> crate::error::Result<Self> {
        if !check_texture_size(params.width) || !check_texture_size(params.height) {
            return Err(crate::Error::InvalidSize);
        }

        // Don't currently support cube maps, not sure what the best way to do that is
        if params.mode == Mode::CubeMap || params.mode == Mode::ShadowCube {
            return Err(crate::Error::FailedToInitialize);
        }

        let format = params.format;
        let in_vram = params.on_vram;
        let params: citro3d_sys::C3D_TexInitParams = params.into();

        // SAFETY: C3D_Tex is only initialised here after citro3d_sys::C3d_TexInitWithParams returns success,
        // and is properly cleaned up with citro3d::C3D_TexDelete on drop
        unsafe {
            let mut c3d_tex: MaybeUninit<citro3d_sys::C3D_Tex> = core::mem::zeroed();

            let success = citro3d_sys::C3D_TexInitWithParams(
                c3d_tex.as_mut_ptr(),
                core::ptr::null_mut(),
                params,
            );

            if !success {
                return Err(crate::Error::FailedToInitialize);
            }

            let mut tex = Texture {
                tex: c3d_tex.assume_init(),
                format,
                in_vram,
            };

            // Set a default filter, as it won't render properly without one
            tex.set_filter(Filter::Linear, Filter::Nearest);

            Ok(tex)
        }
    }

    /// Upload the provided data buffer to the texture, and to the given face if it's a cube
    /// texture. For flat textures `Face::default()` or `Face::TEX2D` can be used.
    #[doc(alias = "C3D_TexUpload")]
    pub fn load_image(&mut self, data: &[u8], face: Face) -> crate::Result<()> {
        self.load_image_at_mipmap_level(data, face, 0)
    }

    /// Upload the provided data buffer to the texture's specific mipmap level, and to the given
    /// face if it's a cube texture. For flat textures `Face::default()` or `Face::TEX2D` can be used.
    #[doc(alias = "C3D_TexLoadImage")]
    pub fn load_image_at_mipmap_level(
        &mut self,
        data: &[u8],
        face: Face,
        mipmap_level: u8,
    ) -> crate::Result<()> {
        let size = unsafe {
            if mipmap_level > 0 {
                citro3d_sys::C3D_TexCalcLevelSize(
                    self.format.bits_per_pixel() as u32,
                    mipmap_level as i32,
                )
            } else {
                citro3d_sys::C3D_TexCalcTotalSize(
                    self.format.bits_per_pixel() as u32,
                    self.max_level() as i32,
                )
            }
        };

        if data.len() < size as usize {
            return Err(crate::Error::InvalidSize);
        }

        // SAFETY: The `data` buffer has been verified to be long enough
        unsafe {
            citro3d_sys::C3D_TexLoadImage(
                self.as_raw(),
                data.as_ptr() as *const _,
                face as u8,
                mipmap_level as i32,
            );
        }

        Ok(())
    }

    /// Binds this texture to the given texture unit of the GPU.
    ///
    /// SAFETY: This texture must stay alive as long as it's bound to the GPU (and a texenv is using that TexUnit?)
    pub(crate) unsafe fn bind(&self, unit: Unit) {
        unsafe { citro3d_sys::C3D_TexBind(unit as _, &self.tex as *const _ as *mut _) };
    }

    /// Generate a mipmap for this texture, and this face if it's a cube texture.
    /// For flat textures `Face::default()` or `Face::TEX2D` can be used.
    pub fn generate_mipmap(&mut self, face: Face) {
        unsafe {
            citro3d_sys::C3D_TexGenerateMipmap(&mut self.tex as *mut _, face as u8);
        }
    }

    pub fn set_filter(&mut self, mag_filter: Filter, min_filter: Filter) {
        unsafe { citro3d_sys::C3D_TexSetFilter(self.as_raw(), mag_filter as u8, min_filter as u8) };
    }

    pub fn set_filter_mipmap(&mut self, filter: Filter) {
        unsafe {
            citro3d_sys::C3D_TexSetFilterMipmap(self.as_raw(), filter as u8);
        }
    }

    pub fn set_wrap(&mut self, wrap_s: Wrap, wrap_t: Wrap) {
        unsafe {
            citro3d_sys::C3D_TexSetWrap(self.as_raw(), wrap_s as u8, wrap_t as u8);
        }
    }

    pub fn set_lod_bias(&mut self, lod_bias: f32) {
        unsafe {
            citro3d_sys::C3D_TexSetLodBias(self.as_raw(), lod_bias);
        }
    }

    pub fn width(&self) -> u16 {
        unsafe { self.tex.__bindgen_anon_2.__bindgen_anon_1.width }
    }

    pub fn height(&self) -> u16 {
        unsafe { self.tex.__bindgen_anon_2.__bindgen_anon_1.height }
    }

    pub fn param(&self) -> u32 {
        self.tex.param
    }

    pub fn format(&self) -> ColorFormat {
        self.format
    }

    pub fn lod_bias(&self) -> u16 {
        unsafe { self.tex.__bindgen_anon_3.__bindgen_anon_1.lodBias }
    }

    pub fn max_level(&self) -> u8 {
        unsafe { self.tex.__bindgen_anon_3.__bindgen_anon_1.maxLevel }
    }

    pub fn min_level(&self) -> u8 {
        unsafe { self.tex.__bindgen_anon_3.__bindgen_anon_1.minLevel }
    }

    fn as_raw(&self) -> *mut citro3d_sys::C3D_Tex {
        &self.tex as *const _ as *mut _
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        // SAFETY: self.tex was initialised with C3D_TexInitWithParams
        unsafe { citro3d_sys::C3D_TexDelete(self.as_raw()) }
    }
}

fn check_texture_size(size: u16) -> bool {
    if size < MIN_TEX_SIZE || size > MAX_TEX_SIZE {
        return false;
    }

    if (size & (size - 1)) > 0 {
        return false;
    }

    true
}
