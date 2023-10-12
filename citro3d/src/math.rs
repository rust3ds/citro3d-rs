//! Safe wrappers for working with matrix and vector types provided by `citro3d`.

use std::mem::MaybeUninit;

mod projection;

pub use projection::{Orthographic, Perspective, Projection};

/// A 4-vector of `u8`s.
#[doc(alias = "C3D_IVec")]
pub struct IVec(citro3d_sys::C3D_IVec);

/// A 4-vector of `f32`s.
#[doc(alias = "C3D_FVec")]
pub struct FVec(citro3d_sys::C3D_FVec);

/// A quaternion, internally represented the same way as [`FVec`].
#[doc(alias = "C3D_FQuat")]
pub struct FQuat(citro3d_sys::C3D_FQuat);

/// A 4x4 row-major matrix of `f32`s.
#[doc(alias = "C3D_Mtx")]
pub struct Matrix(citro3d_sys::C3D_Mtx);

impl Matrix {
    /// Construct the zero matrix.
    #[doc(alias = "Mtx_Zeros")]
    pub fn zero() -> Self {
        // TODO: should this also be Default::default()?
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Zeros(out.as_mut_ptr());
            Self(out.assume_init())
        }
    }

    /// Construct the identity matrix.
    #[doc(alias = "Mtx_Identity")]
    pub fn identity() -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Identity(out.as_mut_ptr());
            Self(out.assume_init())
        }
    }

    pub(crate) fn as_raw(&self) -> *const citro3d_sys::C3D_Mtx {
        &self.0
    }
}

// region: Projection configuration
//
// TODO: maybe move into `mod projection`, or hoist `projection::*` into here.
// it will probably mostly depend on how big all the matrices/vec impls get.
// Also worth considering is whether `mod projection` should be pub.

/// The [orientation](https://en.wikipedia.org/wiki/Orientation_(geometry))
/// (or "handedness") of the coordinate system. Coordinates are always +Y-up,
/// +X-right.
#[derive(Clone, Copy, Debug)]
pub enum CoordinateOrientation {
    /// A left-handed coordinate system. +Z points into the screen.
    LeftHanded,
    /// A right-handed coordinate system. +Z points out of the screen.
    RightHanded,
}

impl CoordinateOrientation {
    pub(crate) fn is_left_handed(self) -> bool {
        matches!(self, Self::LeftHanded)
    }
}

impl Default for CoordinateOrientation {
    /// This is an opinionated default, but [`RightHanded`](Self::RightHanded)
    /// seems to be the preferred coordinate system for most
    /// [examples](https://github.com/devkitPro/3ds-examples)
    /// from upstream, and is also fairly common in other applications.
    fn default() -> Self {
        Self::RightHanded
    }
}

/// Whether to rotate a projection to account for the 3DS screen orientation.
/// Both screens on the 3DS are oriented such that the "top-left" of the screen
/// in framebuffer coordinates is the physical bottom-left of the screen
/// (i.e. the "width" is smaller than the "height").
#[derive(Clone, Copy, Debug)]
pub enum ScreenOrientation {
    /// Rotate 90° clockwise to account for the 3DS screen rotation. Most
    /// applications will use this variant.
    Rotated,
    /// Do not apply any extra rotation to the projection.
    None,
}

impl Default for ScreenOrientation {
    fn default() -> Self {
        Self::Rotated
    }
}

/// Configuration for calculating stereoscopic projections.
// TODO: not totally happy with this name + API yet, but it works for now.
#[derive(Clone, Copy, Debug)]
pub struct StereoDisplacement {
    /// The horizontal offset of the eye from center. Negative values
    /// correspond to the left eye, and positive values to the right eye.
    pub displacement: f32,
    /// The position of the screen, which determines the focal length. Objects
    /// closer than this depth will appear to pop out of the screen, and objects
    /// further than this will appear inside the screen.
    pub screen_depth: f32,
}

impl StereoDisplacement {
    /// Construct displacement for the left and right eyes simulataneously.
    /// The given `interocular_distance` describes the distance between the two
    /// rendered "eyes". A negative value will be treated the same as a positive
    /// value of the same magnitude.
    ///
    /// See struct documentation for details about the
    /// [`screen_depth`](Self::screen_depth) parameter.
    pub fn new(interocular_distance: f32, screen_depth: f32) -> (Self, Self) {
        let displacement = interocular_distance.abs() / 2.0;

        let left_eye = Self {
            displacement: -displacement,
            screen_depth,
        };
        let right_eye = Self {
            displacement,
            screen_depth,
        };

        (left_eye, right_eye)
    }
}

/// Configuration for the clipping planes of a projection.
///
/// For [`Perspective`] projections, this is used for the near and far clip planes
/// of the [view frustum](https://en.wikipedia.org/wiki/Viewing_frustum).
///
/// For [`Orthographic`] projections, this is used for the Z clipping planes of
/// the projection.
///
/// Note that the `near` value should always be less than `far`, regardless of
/// [`CoordinateOrientation`]. In other words, these values will be negated
/// when used with a [`RightHanded`](CoordinateOrientation::RightHanded)
/// orientation.
#[derive(Clone, Copy, Debug)]
pub struct ClipPlanes {
    /// The Z-depth of the near clip plane, usually close or equal to zero.
    pub near: f32,
    /// The Z-depth of the far clip plane, usually greater than zero.
    pub far: f32,
}

/// The aspect ratio of a projection plane.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum AspectRatio {
    /// The aspect ratio of the 3DS' top screen (per-eye).
    #[doc(alias = "C3D_AspectRatioTop")]
    TopScreen,
    /// The aspect ratio of the 3DS' bottom screen.
    #[doc(alias = "C3D_AspectRatioBot")]
    BottomScreen,
    /// A custom aspect ratio (should be calcualted as `width / height`).
    Other(f32),
}

impl From<AspectRatio> for f32 {
    fn from(ratio: AspectRatio) -> Self {
        match ratio {
            AspectRatio::TopScreen => citro3d_sys::C3D_AspectRatioTop as f32,
            AspectRatio::BottomScreen => citro3d_sys::C3D_AspectRatioBot as f32,
            AspectRatio::Other(ratio) => ratio,
        }
    }
}

// endregion