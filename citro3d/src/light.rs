//! Bindings for accessing the lighting part of the GPU pipeline
//!
//! The hardware at play is shown in [this diagram][hardware], you should probably have
//! it open as a reference for the documentation in this module.
//!
//! # Hardware lights
//! There are 8 lights in the GPU's pipeline each of which have 4 colour fields and 1 spotlight colour,
//! you can set all of them at once with [`LightEnv::set_material`]. When rendering for example you call
//! `set_material` in your preparation code before the actual draw call.
//!
//! For things like specular lighting we need to go a bit deeper
//!
//! # LUTS
//! LUTS are lookup tables, in this case for the GPU. They are created ahead of time and stored in [`LightLut`]'s,
//! [`LightLut::from_fn`] essentially memoises the given function with the input changing depending on what
//! input it is bound to when setting it on the [`LightEnv`].
//!
//! ## Example
//! Lets say we have this code
//!
//! ```
//! # use citro3d::{Instance, light::{LightLutId, LightInput, LightLut}};
//! let mut inst = Instance::new();
//! let mut env = inst.light_env_mut();
//! env.as_mut().connect_lut(
//!     LutInputId::D0,
//!     LutInput::NormalView,
//!     LightLut::from_fn(|x| x.powf(10.0)),
//! );
//! ```
//!
//! This places the LUT in `D0` (refer to [the diagram][hardware]) and connects the input wire as the dot product
//! of the normal and view vectors. `x` is effectively the dot product of the normal and view for every vertex and
//! the return of the closure goes out on the corresponding wire
//! (which in the case of `D0` is used for specular lighting after being combined with with specular0)
//!
//!
//!
//! [hardware]: https://raw.githubusercontent.com/wwylele/misc-3ds-diagram/master/pica-pipeline.svg

use std::{marker::PhantomPinned, mem::MaybeUninit, ops::Range, pin::Pin};

use pin_array::PinArray;

use crate::{
    material::Material,
    math::{FVec3, FVec4},
};

/// Index for one of the 8 hardware lights in the GPU pipeline
///
/// Usually you don't want to construct one of these directly but use [`LightEnv::create_light`]
// Note we use a u8 here since usize is overkill and it saves a few bytes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightIndex(u8);

const NB_LIGHTS: usize = 8;

impl LightIndex {
    /// Manually create a `LightIndex` with a specific index
    ///
    /// # Panics
    /// if `idx` out of range for the number of lights (>=8)
    pub fn new(idx: usize) -> Self {
        assert!(idx < NB_LIGHTS);
        Self(idx as u8)
    }
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

type LightArray = PinArray<Option<Light>, NB_LIGHTS>;

pub struct LightEnv {
    raw: citro3d_sys::C3D_LightEnv,
    /// The actual light data pointed to by the lights element of `raw`
    ///
    /// Note this is `Pin` as well, because `raw` means we are _actually_ self-referential which
    /// is horrible but the best bad option in this case. Moving the one of these elements would
    /// break the pointers in `raw`
    lights: LightArray,
    luts: [Option<LightLut>; 6],
    _pin: PhantomPinned,
}

pub struct Light {
    raw: citro3d_sys::C3D_Light,
    // todo: implement spotlight support
    _spot: Option<LightLut>,
    diffuse_atten: Option<LightLutDistAtten>,
    _pin: PhantomPinned,
}

impl Default for LightEnv {
    fn default() -> Self {
        let raw = unsafe {
            let mut env = MaybeUninit::zeroed();
            citro3d_sys::C3D_LightEnvInit(env.as_mut_ptr());
            env.assume_init()
        };
        Self {
            raw,
            lights: Default::default(),
            luts: Default::default(),
            _pin: Default::default(),
        }
    }
}
impl LightEnv {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_material(self: Pin<&mut Self>, mat: Material) {
        let raw = mat.to_raw();
        // Safety: This takes a pointer but it actually memcpy's it so this doesn't dangle
        unsafe {
            citro3d_sys::C3D_LightEnvMaterial(self.as_raw_mut() as *mut _, (&raw) as *const _);
        }
    }

    pub fn lights(&self) -> &LightArray {
        &self.lights
    }

    pub fn lights_mut(self: Pin<&mut Self>) -> Pin<&mut LightArray> {
        unsafe { self.map_unchecked_mut(|s| &mut s.lights) }
    }

    pub fn light_mut(self: Pin<&mut Self>, idx: LightIndex) -> Option<Pin<&mut Light>> {
        self.lights_mut()
            .get_pin(idx.0 as usize)
            .unwrap()
            .as_pin_mut()
    }
    pub fn create_light(mut self: Pin<&mut Self>) -> Option<LightIndex> {
        let idx = self
            .lights()
            .iter()
            .enumerate()
            .find(|(_, n)| n.is_none())
            .map(|(n, _)| n)?;

        self.as_mut()
            .lights_mut()
            .get_pin(idx)
            .unwrap()
            .set(Some(Light::new(unsafe {
                MaybeUninit::zeroed().assume_init()
            })));

        let target = unsafe {
            self.as_mut()
                .lights_mut()
                .get_pin(idx)
                .unwrap()
                .map_unchecked_mut(|p| p.as_mut().unwrap())
        };
        let r = unsafe {
            citro3d_sys::C3D_LightInit(
                target.get_unchecked_mut().as_raw_mut(),
                self.as_raw_mut() as *mut _,
            )
        };
        assert!(r >= 0, "C3D_LightInit should only fail if there are no free light slots but we checked that already, how did this happen?");
        assert_eq!(
            r as usize, idx,
            "citro3d chose a different light to us? this shouldn't be possible"
        );
        Some(LightIndex::new(idx))
    }
    fn lut_id_to_index(id: LightLutId) -> Option<usize> {
        match id {
            LightLutId::D0 => Some(0),
            LightLutId::D1 => Some(1),
            LightLutId::SpotLightAttenuation => None,
            LightLutId::Fresnel => Some(2),
            LightLutId::ReflectBlue => Some(3),
            LightLutId::ReflectGreen => Some(4),
            LightLutId::ReflectRed => Some(5),
            LightLutId::DistanceAttenuation => None,
        }
    }
    /// Attempt to disconnect a light lut
    ///
    /// # Note
    /// This function will not panic if the lut does not exist for `id` and `input`, it will just return `None`
    pub fn disconnect_lut(
        mut self: Pin<&mut Self>,
        id: LightLutId,
        input: LutInput,
    ) -> Option<LightLut> {
        let idx = Self::lut_id_to_index(id);
        let me = unsafe { self.as_mut().get_unchecked_mut() };
        let lut = idx.and_then(|i| me.luts[i].take());
        if lut.is_some() {
            unsafe {
                citro3d_sys::C3D_LightEnvLut(
                    &mut me.raw,
                    id as u8,
                    input as u8,
                    false,
                    std::ptr::null_mut(),
                );
            }
        }
        lut
    }
    pub fn connect_lut(mut self: Pin<&mut Self>, id: LightLutId, input: LutInput, data: LightLut) {
        let idx = Self::lut_id_to_index(id);
        let (raw, lut) = unsafe {
            // this is needed to do structural borrowing as otherwise
            // the compiler rejects the reborrow needed with the pin
            let me = self.as_mut().get_unchecked_mut();
            let lut = idx.map(|i| me.luts[i].insert(data));
            let raw = &mut me.raw;
            let lut = match lut {
                Some(l) => (&mut l.0) as *mut _,
                None => core::ptr::null_mut(),
            };
            (raw, lut)
        };
        unsafe {
            citro3d_sys::C3D_LightEnvLut(raw, id as u8, input as u8, false, lut);
        }
    }
    pub fn set_fresnel(self: Pin<&mut Self>, sel: FresnelSelector) {
        unsafe { citro3d_sys::C3D_LightEnvFresnel(self.as_raw_mut(), sel as _) }
    }

    pub fn as_raw(&self) -> &citro3d_sys::C3D_LightEnv {
        &self.raw
    }

    pub fn as_raw_mut(self: Pin<&mut Self>) -> &mut citro3d_sys::C3D_LightEnv {
        unsafe { &mut self.get_unchecked_mut().raw }
    }
}

impl Light {
    fn new(raw: citro3d_sys::C3D_Light) -> Self {
        Self {
            raw,
            _spot: Default::default(),
            diffuse_atten: Default::default(),
            _pin: Default::default(),
        }
    }

    /// Get a reference to the underlying raw `C3D_Light`
    pub fn as_raw(&self) -> &citro3d_sys::C3D_Light {
        &self.raw
    }

    /// Get a raw mut to the raw `C3D_Light`
    ///
    /// note: This does not take Pin<&mut Self>, if you need the raw from a pinned light you must use `unsafe` and ensure you uphold the pinning
    /// restrictions of the original `Light`
    pub fn as_raw_mut(&mut self) -> &mut citro3d_sys::C3D_Light {
        &mut self.raw
    }

    pub fn set_position(self: Pin<&mut Self>, p: FVec3) {
        let mut p = FVec4::new(p.x(), p.y(), p.z(), 1.0);
        unsafe { citro3d_sys::C3D_LightPosition(self.get_unchecked_mut().as_raw_mut(), &mut p.0) }
    }
    pub fn set_color(self: Pin<&mut Self>, r: f32, g: f32, b: f32) {
        unsafe { citro3d_sys::C3D_LightColor(self.get_unchecked_mut().as_raw_mut(), r, g, b) }
    }
    #[doc(alias = "C3D_LightEnable")]
    pub fn set_enabled(self: Pin<&mut Self>, enabled: bool) {
        unsafe { citro3d_sys::C3D_LightEnable(self.get_unchecked_mut().as_raw_mut(), enabled) }
    }
    #[doc(alias = "C3D_LightShadowEnable")]
    pub fn set_shadow(self: Pin<&mut Self>, shadow: bool) {
        unsafe { citro3d_sys::C3D_LightShadowEnable(self.get_unchecked_mut().as_raw_mut(), shadow) }
    }
    pub fn set_distance_attenutation(mut self: Pin<&mut Self>, lut: Option<LightLutDistAtten>) {
        {
            let me = unsafe { self.as_mut().get_unchecked_mut() };
            me.diffuse_atten = lut;
        }
        // this is a bit of a mess because we need to be _reallly_ careful we don't trip aliasing rules
        // reusing `me` here I think trips them because we have multiple live mutable references to
        // the same region
        let (raw, c_lut) = {
            let me = unsafe { self.as_mut().get_unchecked_mut() };
            let raw = &mut me.raw;
            let c_lut = me.diffuse_atten.as_mut().map(|d| &mut d.raw);
            (raw, c_lut)
        };
        unsafe {
            citro3d_sys::C3D_LightDistAttn(
                raw,
                match c_lut {
                    Some(l) => l,
                    None => std::ptr::null_mut(),
                },
            );
        }
    }
}

// Safety: I am 99% sure these are safe. That 1% is if citro3d does something weird I missed
// which is not impossible
unsafe impl Send for Light {}
unsafe impl Sync for Light {}

unsafe impl Send for LightEnv {}
unsafe impl Sync for LightEnv {}

/// Lookup table for light data
///
/// For more refer to the module documentation
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct LightLut(citro3d_sys::C3D_LightLut);

impl PartialEq for LightLut {
    fn eq(&self, other: &Self) -> bool {
        self.0.data == other.0.data
    }
}
impl Eq for LightLut {}

impl std::hash::Hash for LightLut {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.data.hash(state);
    }
}

#[cfg(test)]
extern "C" fn c_powf(a: f32, b: f32) -> f32 {
    a.powf(b)
}

type LutArray = [u32; 256];
const LUT_BUF_SZ: usize = 512;

impl LightLut {
    /// Create a LUT by memoizing a function
    pub fn from_fn(mut f: impl FnMut(f32) -> f32, negative: bool) -> Self {
        let base: i32 = 128;
        let diff = if negative { 0 } else { base };
        let min = -128 + diff;
        let max = base + diff;
        assert_eq!(min.abs_diff(max), 2 * base as u32);
        let mut data = [0.0f32; LUT_BUF_SZ];
        for i in min..=max {
            let x = i as f32 / max as f32;
            let v = f(x);
            let idx = if negative { i & 0xFF } else { i } as usize;
            if i < max {
                data[idx] = v;
            }
            if i > min {
                data[idx + 255] = v - data[idx - 1];
            }
        }
        let lut = unsafe {
            let mut lut = MaybeUninit::zeroed();
            citro3d_sys::LightLut_FromArray(lut.as_mut_ptr(), data.as_mut_ptr());
            lut.assume_init()
        };
        Self(lut)
    }

    /// Get a reference to the underlying data
    pub fn data(&self) -> &LutArray {
        &self.0.data
    }

    /// Get a mutable reference to the underlying data
    pub fn data_mut(&mut self) -> &mut LutArray {
        &mut self.0.data
    }

    #[cfg(test)]
    fn phong_citro3d(shininess: f32) -> Self {
        let lut = unsafe {
            let mut lut = MaybeUninit::uninit();
            citro3d_sys::LightLut_FromFunc(lut.as_mut_ptr(), Some(c_powf), shininess, false);
            lut.assume_init()
        };
        Self(lut)
    }
}

pub struct LightLutDistAtten {
    raw: citro3d_sys::C3D_LightLutDA,
}

impl LightLutDistAtten {
    pub fn new(range: Range<f32>, mut f: impl FnMut(f32) -> f32) -> Self {
        let mut raw: citro3d_sys::C3D_LightLutDA = unsafe { MaybeUninit::zeroed().assume_init() };
        let dist = range.end - range.start;
        raw.scale = 1.0 / dist;
        raw.bias = -range.start * raw.scale;
        let lut = LightLut::from_fn(|x| f(range.start + dist * x), false);
        raw.lut = citro3d_sys::C3D_LightLut { data: *lut.data() };
        Self { raw }
    }
}

/// This is used to decide what the input should be to a [`LightLut`]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum LutInput {
    CosPhi = ctru_sys::GPU_LUTINPUT_CP,
    /// Dot product of the light and normal vectors
    LightNormal = ctru_sys::GPU_LUTINPUT_LN,
    /// Half the normal
    NormalHalf = ctru_sys::GPU_LUTINPUT_NH,
    /// Dot product of the view and normal
    NormalView = ctru_sys::GPU_LUTINPUT_NV,
    /// Dot product of the spotlight colour and light vector
    LightSpotLight = ctru_sys::GPU_LUTINPUT_SP,
    /// Half the view vector
    ViewHalf = ctru_sys::GPU_LUTINPUT_VH,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum LightLutId {
    D0 = ctru_sys::GPU_LUT_D0,
    D1 = ctru_sys::GPU_LUT_D1,
    SpotLightAttenuation = ctru_sys::GPU_LUT_SP,
    Fresnel = ctru_sys::GPU_LUT_FR,
    ReflectBlue = ctru_sys::GPU_LUT_RB,
    ReflectGreen = ctru_sys::GPU_LUT_RG,
    ReflectRed = ctru_sys::GPU_LUT_RR,
    DistanceAttenuation = ctru_sys::GPU_LUT_DA,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum FresnelSelector {
    /// No fresnel selection
    None = ctru_sys::GPU_NO_FRESNEL,
    /// Use as selector for primary colour unit alpha
    PrimaryAlpha = ctru_sys::GPU_PRI_ALPHA_FRESNEL,
    /// Use as selector for secondary colour unit alpha
    SecondaryAlpha = ctru_sys::GPU_SEC_ALPHA_FRESNEL,
    /// Use as selector for both colour units
    Both = ctru_sys::GPU_PRI_SEC_ALPHA_FRESNEL,
}

#[cfg(test)]
mod tests {
    use super::LightLut;

    #[test]
    fn lut_data_phong_matches_for_own_and_citro3d() {
        let c3d = LightLut::phong_citro3d(30.0);
        let rs = LightLut::from_fn(|i| i.powf(30.0), false);
        assert_eq!(c3d, rs);
    }
}
