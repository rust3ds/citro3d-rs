//! Safe wrappers for working with matrix and vector types provided by `citro3d`.

use std::mem::MaybeUninit;

use crate::AspectRatio;

/// A 4-vector of [`u8`]s.
pub struct IntVec(citro3d_sys::C3D_IVec);

/// A 4-vector of [`f32`]s.
pub struct FloatVec(citro3d_sys::C3D_FVec);

/// A quaternion, internally represented the same way as [`FVec`].
pub struct Quaternion(citro3d_sys::C3D_FQuat);

/// A 4x4 row-major matrix of [`f32`]s.
pub struct Matrix(citro3d_sys::C3D_Mtx);

/// Whether to use left-handed or right-handed coordinates for calculations.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive] // This probably is exhaustive, but just in case
pub enum CoordinateSystem {
    LeftHanded,
    RightHanded,
}

impl Matrix {
    // TODO: this could probably be generalized with something like builder or options
    // pattern. Should look and see what the different citro3d implementations look like
    pub fn perspective_stereo_tilt(
        fov_y: f32,
        aspect_ratio: AspectRatio,
        near: f32,
        far: f32,
        interocular_distance: f32,
        /* better name ?? */ screen_depth: f32,
        coordinates: CoordinateSystem,
    ) -> Self {
        let mut result = MaybeUninit::uninit();

        let inner = unsafe {
            citro3d_sys::Mtx_PerspStereoTilt(
                result.as_mut_ptr(),
                fov_y,
                aspect_ratio.into(),
                near,
                far,
                interocular_distance,
                screen_depth,
                matches!(coordinates, CoordinateSystem::LeftHanded),
            );

            result.assume_init()
        };

        Self(inner)
    }

    pub fn perspective_tilt(
        fov_y: f32,
        aspect_ratio: AspectRatio,
        near: f32,
        far: f32,
        coordinates: CoordinateSystem,
    ) -> Self {
        let mut result = MaybeUninit::uninit();

        let inner = unsafe {
            citro3d_sys::Mtx_PerspTilt(
                result.as_mut_ptr(),
                fov_y,
                aspect_ratio.into(),
                near,
                far,
                matches!(coordinates, CoordinateSystem::LeftHanded),
            );

            result.assume_init()
        };

        Self(inner)
    }

    pub(crate) fn as_raw(&self) -> *const citro3d_sys::C3D_Mtx {
        &self.0
    }
}
