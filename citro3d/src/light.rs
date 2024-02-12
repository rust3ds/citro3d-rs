//! Bindings for accessing the lighting part of the GPU pipeline
//!
//! What does anything in this module mean? inspect this diagram: https://raw.githubusercontent.com/wwylele/misc-3ds-diagram/master/pica-pipeline.svg

use std::{mem::MaybeUninit, pin::Pin};

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
}

#[derive(Default)]
struct LightEnvStorage {
    lights: [Option<Light>; NB_LIGHTS],
    luts: [Option<LutData>; 6],
}

pub struct LightEnv {
    raw: citro3d_sys::C3D_LightEnv,
    /// The actual light data pointed to by the lights element of `raw`
    ///
    /// Note this is `Pin` as well as `Box` as `raw` means we are _actually_ self-referential which
    /// is horrible but the best bad option in this case
    store: Pin<Box<LightEnvStorage>>,
}

pub struct Light(citro3d_sys::C3D_Light);

impl Default for LightEnv {
    fn default() -> Self {
        let raw = unsafe {
            let mut env = MaybeUninit::uninit();
            citro3d_sys::C3D_LightEnvInit(env.as_mut_ptr());
            env.assume_init()
        };
        Self {
            raw,
            store: Pin::new(Default::default()),
        }
    }
}
impl LightEnv {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_material(&mut self, mat: Material) {
        let raw = mat.to_raw();
        // Safety: This takes a pointer but it actually memcpy's it so this doesn't dangle
        unsafe {
            citro3d_sys::C3D_LightEnvMaterial(self.as_raw_mut() as *mut _, (&raw) as *const _);
        }
    }

    pub fn lights(&self) -> [Option<&Light>; 8] {
        core::array::from_fn(|i| self.store.lights[i].as_ref())
    }

    pub fn light_mut(&mut self, idx: LightIndex) -> Option<&mut Light> {
        self.store.lights[idx.0 as usize].as_mut()
    }
    pub fn create_light(&mut self) -> Option<LightIndex> {
        let idx = self
            .lights()
            .iter()
            .enumerate()
            .find(|(_, n)| n.is_none())
            .map(|(n, _)| n)?;

        self.store.lights[idx] = Some(Light(unsafe { MaybeUninit::zeroed().assume_init() }));

        let r = unsafe {
            citro3d_sys::C3D_LightInit(
                self.store.lights[idx].as_mut().unwrap().as_raw_mut(),
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
    ///
    pub fn connect_lut(&mut self, id: LightLutId, input: LutInput, data: LutData) {
        let idx = match id {
            LightLutId::D0 => Some(0),
            LightLutId::D1 => Some(1),
            LightLutId::SpotLightAttenuation => None,
            LightLutId::Fresnel => Some(2),
            LightLutId::ReflectBlue => Some(3),
            LightLutId::ReflectGreen => Some(4),
            LightLutId::ReflectRed => Some(5),
            LightLutId::DistanceAttenuation => None,
        };
        let lut = idx.map(|i| self.store.luts[i].insert(data));
        let raw = &mut self.raw;
        let lut = match lut {
            Some(l) => (&mut l.0) as *mut _,
            None => core::ptr::null_mut(),
        };
        unsafe {
            citro3d_sys::C3D_LightEnvLut(raw, id as u32, input as u32, false, lut);
        }
    }

    pub fn as_raw(&self) -> &citro3d_sys::C3D_LightEnv {
        &self.raw
    }

    pub fn as_raw_mut(&mut self) -> &mut citro3d_sys::C3D_LightEnv {
        &mut self.raw
    }
}

impl Light {
    fn from_raw_ref(l: &citro3d_sys::C3D_Light) -> &Self {
        unsafe { (l as *const _ as *const Self).as_ref().unwrap() }
    }
    fn from_raw_mut(l: &mut citro3d_sys::C3D_Light) -> &mut Self {
        unsafe { (l as *mut _ as *mut Self).as_mut().unwrap() }
    }
    fn as_raw(&self) -> &citro3d_sys::C3D_Light {
        &self.0
    }

    fn as_raw_mut(&mut self) -> &mut citro3d_sys::C3D_Light {
        &mut self.0
    }
    pub fn set_position(&mut self, p: FVec3) {
        let mut p = FVec4::new(p.x(), p.y(), p.z(), 1.0);
        unsafe { citro3d_sys::C3D_LightPosition(self.as_raw_mut(), &mut p.0) }
    }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        unsafe { citro3d_sys::C3D_LightColor(self.as_raw_mut(), r, g, b) }
    }
    #[doc(alias = "C3D_LightEnable")]
    pub fn set_enabled(&mut self, enabled: bool) {
        unsafe { citro3d_sys::C3D_LightEnable(self.as_raw_mut(), enabled) }
    }
    #[doc(alias = "C3D_LightShadowEnable")]
    pub fn set_shadow(&mut self, shadow: bool) {
        unsafe { citro3d_sys::C3D_LightShadowEnable(self.as_raw_mut(), shadow) }
    }
}

// Safety: I am 99% sure these are safe. That 1% is if citro3d does something weird I missed
// which is not impossible
unsafe impl Send for Light {}
unsafe impl Sync for Light {}

unsafe impl Send for LightEnv {}
unsafe impl Sync for LightEnv {}

#[repr(transparent)]
pub struct LutData(citro3d_sys::C3D_LightLut);

extern "C" fn c_powf(a: f32, b: f32) -> f32 {
    a.powf(b)
}

impl LutData {
    pub fn phong(shininess: f32) -> Self {
        let lut = unsafe {
            let mut lut = MaybeUninit::uninit();
            citro3d_sys::LightLut_FromFunc(lut.as_mut_ptr(), Some(c_powf), shininess, false);
            lut.assume_init()
        };
        Self(lut)
    }
}

#[repr(u32)]
pub enum LutInput {
    CosPhi = ctru_sys::GPU_LUTINPUT_CP,
    /// Light vector * normal
    LightNormal = ctru_sys::GPU_LUTINPUT_LN,
    /// normal * half vector
    NormalHalf = ctru_sys::GPU_LUTINPUT_NH,
    /// normal * view
    NormalView = ctru_sys::GPU_LUTINPUT_NV,
    /// light * spotlight
    LightSpotLight = ctru_sys::GPU_LUTINPUT_SP,
    /// view * half vector
    ViewHalf = ctru_sys::GPU_LUTINPUT_VH,
}

#[repr(u32)]
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
