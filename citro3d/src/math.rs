//! Safe wrappers for working with matrix and vector types provided by `citro3d`.

use std::mem::MaybeUninit;

use crate::AspectRatio;

/// A 4-vector of `u8`s.
pub struct IntVec(citro3d_sys::C3D_IVec);

/// A 4-vector of `f32`s.
pub struct FloatVec(citro3d_sys::C3D_FVec);

/// A quaternion, internally represented the same way as [`FloatVec`].
pub struct Quaternion(citro3d_sys::C3D_FQuat);

/// A 4x4 row-major matrix of `f32`s.
pub struct Matrix(citro3d_sys::C3D_Mtx);

impl Matrix {
    // TODO: does it make sense to have a helper that builds both left and right
    // eyes for stereoscopic at the same time?

    /// Construct a projection matrix suitable for projecting 3D world space onto
    /// the 3DS screens.
    pub fn perspective_projection(
        vertical_fov: f32,
        aspect_ratio: AspectRatio,
        orientation: Orientation,
        clip_plane: ClipPlane,
        stereo: Stereoscopic,
        coordinates: CoordinateSystem,
    ) -> Self {
        let (make_mtx_persp, make_mtx_stereo);

        let initialize_mtx: &dyn Fn(_, _, _, _, _, _) -> _ = match stereo {
            Stereoscopic::Mono => {
                let make_mtx = match orientation {
                    Orientation::Natural => citro3d_sys::Mtx_PerspTilt,
                    Orientation::HardwareDefault => citro3d_sys::Mtx_Persp,
                };

                make_mtx_persp = move |a, b, c, d, e, f| unsafe { make_mtx(a, b, c, d, e, f) };
                &make_mtx_persp
            }
            Stereoscopic::Stereo {
                interocular_distance,
                screen_depth,
            } => {
                let make_mtx = match orientation {
                    Orientation::Natural => citro3d_sys::Mtx_PerspStereoTilt,
                    Orientation::HardwareDefault => citro3d_sys::Mtx_PerspStereo,
                };

                make_mtx_stereo = move |a, b, c, d, e, f| unsafe {
                    make_mtx(a, b, c, d, interocular_distance, screen_depth, e, f)
                };
                &make_mtx_stereo
            }
        };

        let left_handed = matches!(coordinates, CoordinateSystem::LeftHanded);

        let mut result = MaybeUninit::uninit();
        initialize_mtx(
            result.as_mut_ptr(),
            vertical_fov,
            aspect_ratio.into(),
            clip_plane.near,
            clip_plane.far,
            left_handed,
        );

        let inner = unsafe { result.assume_init() };

        Self(inner)
    }

    pub(crate) fn as_raw(&self) -> *const citro3d_sys::C3D_Mtx {
        &self.0
    }
}

/// Whether to use left-handed or right-handed coordinates for calculations.
#[derive(Clone, Copy, Debug)]
pub enum CoordinateSystem {
    LeftHanded,
    RightHanded,
}

/// Whether to rotate a projection to account for the 3DS screen configuration.
/// Both screens on the 3DS are oriented such that the "top" of the screen is
/// on the [left | right] ? side of the device when it's held normally, so
/// projections must account for this extra rotation to display in the correct
/// orientation.
#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    /// Rotate the projection 90° to account for the 3DS screen rotation.
    Natural,
    /// Don't rotate the projection at all.
    HardwareDefault,
}

#[derive(Clone, Copy, Debug)]
// TODO: better name
pub enum Stereoscopic {
    Mono,
    Stereo {
        interocular_distance: f32,
        // TODO: better name? At least docstring
        screen_depth: f32,
    },
}

impl Stereoscopic {
    /// Flip the stereoscopic projection for the opposite eye.
    pub fn invert(self) -> Self {
        match self {
            Self::Stereo {
                interocular_distance,
                screen_depth,
            } => Self::Stereo {
                interocular_distance: -interocular_distance,
                screen_depth,
            },
            mono => mono,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClipPlane {
    pub near: f32,
    pub far: f32,
}
