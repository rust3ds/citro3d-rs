//! Texture environment support. See `<c3d/texenv.h>` for more information.

use bitflags::bitflags;

use crate::Instance;

#[doc(alias = "C3D_TexEnv")]
pub struct TexEnv<'a> {
    raw: *mut citro3d_sys::C3D_TexEnv,
    _instance: &'a mut Instance,
}

impl<'a> TexEnv<'a> {
    #[doc(alias = "C3D_TexEnvInit")]
    pub fn set(&self, _instance: &mut Instance, id: Id) {
        unsafe {
            // SAFETY: pointee is only copied from, not modified
            citro3d_sys::C3D_SetTexEnv(id.0, self.raw);
        }
    }

    pub fn get(instance: &'a mut Instance, id: Id) -> Self {
        unsafe {
            Self {
                raw: citro3d_sys::C3D_GetTexEnv(id.0),
                _instance: instance,
            }
        }
    }

    pub fn src(
        &mut self,
        mode: Mode,
        s1: Source,
        s2: Option<Source>,
        s3: Option<Source>,
    ) -> &mut Self {
        unsafe {
            citro3d_sys::C3D_TexEnvSrc(
                self.raw,
                mode.bits(),
                s1 as _,
                s2.unwrap_or(Source::PrimaryColor) as _,
                s3.unwrap_or(Source::PrimaryColor) as _,
            )
        }
        self
    }

    pub fn func(&mut self, mode: Mode, func: CombineFunc) -> &mut Self {
        unsafe {
            citro3d_sys::C3D_TexEnvFunc(self.raw, mode.bits(), func as _);
        }

        self
    }
}

bitflags! {
    #[doc(alias = "C3D_TexEnvMode")]
    pub struct Mode: citro3d_sys::C3D_TexEnvMode {
        const RGB = citro3d_sys::C3D_RGB;
        const ALPHA = citro3d_sys::C3D_Alpha;
        const BOTH = citro3d_sys::C3D_Both;
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[doc(alias = "GPU_TEVSRC")]
pub enum Source {
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

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[doc(alias = "GPU_COMBINEFUNC")]
pub enum CombineFunc {
    Replace = ctru_sys::GPU_REPLACE,
    Modulate = ctru_sys::GPU_MODULATE,
    Add = ctru_sys::GPU_ADD,
    AddSigned = ctru_sys::GPU_ADD_SIGNED,
    Interpolate = ctru_sys::GPU_INTERPOLATE,
    Subtract = ctru_sys::GPU_SUBTRACT,
    Dot3Rgb = ctru_sys::GPU_DOT3_RGB,
    Dot3Rgba = ctru_sys::GPU_DOT3_RGBA,
}

#[derive(Copy, Clone, Debug)]
pub struct Id(/* TODO maybe non pub? idk */ pub libc::c_int);
