//! Texture combiner support. See <https://www.khronos.org/opengl/wiki/Texture_Combiners>
//! for more details.

use bitflags::bitflags;

// https://oreo639.github.io/citro3d/texenv_8h.html#a9eda91f8e7252c91f873b1d43e3728b6
pub(crate) const TEXENV_COUNT: usize = 6;

/// A texture combiner, also called a "texture environment" (hence the struct name).
/// See also [`texenv.h` documentation](https://oreo639.github.io/citro3d/texenv_8h.html).
#[derive(Clone, Copy)]
#[doc(alias = "C3D_TexEnv")]
pub struct TexEnv {
    inner: citro3d_sys::C3D_TexEnv,

    // For checking that necessary textures are bound
    pub(crate) sources: [Source; 6],
}

impl TexEnv {
    pub fn as_raw(&self) -> *mut citro3d_sys::C3D_TexEnv {
        &self.inner as *const _ as *mut _
    }

    /// Create a new texture combiner stage, or "texture environment"
    #[doc(alias = "C3D_TexEnvInit")]
    pub fn new() -> TexEnv {
        let inner = unsafe {
            let mut inner = core::mem::MaybeUninit::<citro3d_sys::C3D_TexEnv>::uninit();
            Self::init_reset(inner.as_mut_ptr());
            inner.assume_init()
        };

        TexEnv {
            inner,
            sources: [Source::default(); 6],
        }
    }

    #[doc(alias = "C3D_TexEnvInit")]
    pub fn reset(self) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvInit(self.as_raw());
        }

        TexEnv {
            inner: self.inner,
            sources: [Source::default(); 6],
        }
    }

    /// Set the sources to use for the rgb and/or alpha components of this texenv stage.
    /// If sourcing from a texture unit, ensure a texture is also bound to that unit
    /// with [`Frame::with_texture`]
    #[doc(alias = "C3D_TexEnvSrc")]
    pub fn src(
        mut self,
        mode: Mode,
        source0: Source,
        source1: Option<Source>,
        source2: Option<Source>,
    ) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvSrc(
                self.as_raw(),
                mode.bits(),
                source0 as _,
                source1.unwrap_or_default() as _,
                source2.unwrap_or_default() as _,
            );
        }

        if mode.contains(Mode::RGB) {
            self.sources[0] = source0;
            self.sources[1] = source1.unwrap_or_default();
            self.sources[2] = source2.unwrap_or_default();
        }

        if mode.contains(Mode::ALPHA) {
            self.sources[3] = source0;
            self.sources[4] = source1.unwrap_or_default();
            self.sources[5] = source2.unwrap_or_default();
        }

        self
    }

    #[doc(alias = "C3D_TexEnvOpRgb")]
    pub fn op_rgb(self, o1: RGBOp, o2: Option<RGBOp>, o3: Option<RGBOp>) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvOpRgb(
                self.as_raw(),
                o1 as _,
                o2.unwrap_or_default() as _,
                o3.unwrap_or_default() as _,
            );
        }
        self
    }

    #[doc(alias = "C3D_TexEnvOpAlpha")]
    pub fn op_alpha(self, o1: AlphaOp, o2: Option<AlphaOp>, o3: Option<AlphaOp>) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvOpAlpha(
                self.as_raw(),
                o1 as _,
                o2.unwrap_or_default() as _,
                o3.unwrap_or_default() as _,
            );
        }
        self
    }

    #[doc(alias = "C3D_TexEnvFunc")]
    pub fn func(self, mode: Mode, func: CombineFunc) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvFunc(self.as_raw(), mode.bits(), func as _);
        }
        self
    }

    #[doc(alias = "C3D_TexEnvColor")]
    pub fn color(self, color: u32) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvColor(self.as_raw(), color);
        }
        self
    }

    #[doc(alias = "C3D_TexEnvScale")]
    pub fn scale(self, mode: Mode, scale: Scale) -> TexEnv {
        unsafe {
            citro3d_sys::C3D_TexEnvScale(self.as_raw(), mode.bits() as _, scale as _);
        }
        self
    }

    /// Set this as the active texenv in the given stage.
    #[must_use]
    #[doc(alias = "C3D_SetTexEnv")]
    pub(crate) fn set_texenv(&self, stage: usize) -> crate::Result<()> {
        if stage >= TEXENV_COUNT {
            return Err(crate::Error::IndexOutOfBounds {
                idx: stage as i32,
                len: TEXENV_COUNT as i32,
            });
        }

        unsafe {
            citro3d_sys::C3D_SetTexEnv(stage as i32, self.as_raw());
        }

        Ok(())
    }

    #[doc(alias = "C3D_TexEnvInit")]
    pub(crate) unsafe fn init_reset(texenv: *mut citro3d_sys::C3D_TexEnv) {
        unsafe {
            citro3d_sys::C3D_TexEnvInit(texenv);
        }
    }

    #[doc(alias = "C3D_TexEnvGet")]
    pub(crate) unsafe fn get_texenv(stage: usize) -> *mut citro3d_sys::C3D_TexEnv {
        unsafe { citro3d_sys::C3D_GetTexEnv(stage as i32) }
    }
}

/// Configure the source values of the texture combiner.
#[derive(Clone, Copy)]
pub struct Sources {
    /// The first [`Source`] operand to the texture combiner
    pub source0: Source,
    /// Optional additional [`Source`] operand to use
    pub source1: Option<Source>,
    /// Optional additional [`Source`] operand to use
    pub source2: Option<Source>,
}

impl Default for Sources {
    fn default() -> Self {
        Sources {
            source0: Source::PrimaryColor,
            source1: None,
            source2: None,
        }
    }
}

bitflags! {
    /// Whether to operate on colors, alpha values, or both.
    #[doc(alias = "C3D_TexEnvMode")]
    pub struct Mode: citro3d_sys::C3D_TexEnvMode {
        #[allow(missing_docs)]
        const RGB = citro3d_sys::C3D_RGB;
        #[allow(missing_docs)]
        const ALPHA = citro3d_sys::C3D_Alpha;
        #[allow(missing_docs)]
        const BOTH = citro3d_sys::C3D_Both;
    }
}

/// A source operand of a [`TexEnv`]'s texture combination.
#[doc(alias = "GPU_TEVSRC")]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum Source {
    #[default]
    PrimaryColor = ctru_sys::GPU_PRIMARY_COLOR,
    FragmentPrimaryColor = ctru_sys::GPU_FRAGMENT_PRIMARY_COLOR,
    FragmentSecondaryColor = ctru_sys::GPU_FRAGMENT_SECONDARY_COLOR,
    Texture0 = ctru_sys::GPU_TEXTURE0,
    Texture1 = ctru_sys::GPU_TEXTURE1,
    Texture2 = ctru_sys::GPU_TEXTURE2,
    Texture3 = ctru_sys::GPU_TEXTURE3,
    PreviousBuffer = ctru_sys::GPU_PREVIOUS_BUFFER,
    Constant = ctru_sys::GPU_CONSTANT,
    Previous = ctru_sys::GPU_PREVIOUS,
}

/// The combination function to apply to the [`TexEnv`] operands.
#[doc(alias = "GPU_COMBINEFUNC")]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum CombineFunc {
    Replace = ctru_sys::GPU_REPLACE,
    Modulate = ctru_sys::GPU_MODULATE,
    Add = ctru_sys::GPU_ADD,
    AddSigned = ctru_sys::GPU_ADD_SIGNED,
    Interpolate = ctru_sys::GPU_INTERPOLATE,
    Subtract = ctru_sys::GPU_SUBTRACT,
    Dot3Rgb = ctru_sys::GPU_DOT3_RGB,
    // Added in libcrtu 2.3.0:
    // Dot3Rgba = ctru_sys::GPU_DOT3_RGBA,
}

/// The RGB combiner operands.
#[doc(alias = "GPU_TEVOP_RGB")]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum RGBOp {
    #[default]
    SrcColor = ctru_sys::GPU_TEVOP_RGB_SRC_COLOR,
    OneMinusSrcColor = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR,
    SrcAlpha = ctru_sys::GPU_TEVOP_RGB_SRC_ALPHA,
    OneMinusSrcAlpha = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA,
    SrcRed = ctru_sys::GPU_TEVOP_RGB_SRC_R,
    OneMinusSrcRed = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_R,
    SrcGreen = ctru_sys::GPU_TEVOP_RGB_SRC_G,
    OneMinusSrcGreen = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_G,
    SrcBlue = ctru_sys::GPU_TEVOP_RGB_SRC_B,
    OneMinusSrcBlue = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_B,
}

/// The Alpha combiner operands.
#[doc(alias = "GPU_TEVOP_RGB")]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum AlphaOp {
    #[default]
    SrcAlpha = ctru_sys::GPU_TEVOP_A_SRC_ALPHA,
    OneMinusSrcAlpha = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA,
    SrcRed = ctru_sys::GPU_TEVOP_A_SRC_R,
    OneMinusSrcRed = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_R,
    SrcGreen = ctru_sys::GPU_TEVOP_A_SRC_G,
    OneMinusSrcGreen = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_G,
    SrcBlue = ctru_sys::GPU_TEVOP_A_SRC_B,
    OneMinusSrcBlue = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_B,
}

#[doc(alias = "GPU_TEVSCALE")]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Scale {
    #[default]
    X1 = ctru_sys::GPU_TEVSCALE_1,
    X2 = ctru_sys::GPU_TEVSCALE_2,
    X4 = ctru_sys::GPU_TEVSCALE_4,
}
