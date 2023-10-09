//! Safe wrappers for working with matrix and vector types provided by `citro3d`.

use std::mem::MaybeUninit;

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
    /// Construct the identity matrix.
    #[doc(alias = "Mtx_Identity")]
    pub fn identity() -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Identity(out.as_mut_ptr());
            Self(out.assume_init())
        }
    }

    /// Construct a projection matrix suitable for projecting 3D world space onto
    /// the 3DS screens.
    ///
    /// # Parameters
    ///
    /// * `vertical_fov_radians`: the vertical field of view, measured in radians
    /// * `aspect_ratio`: The aspect ratio of the projection
    /// * `orientation`: the orientation of the projection with respect to the screen
    /// * `coordinates`: the handedness of the coordinate system to use
    /// * `stereo`: if specified, the offset to displace the projection by
    ///     for stereoscopic rendering.
    ///
    /// # Examples
    ///
    /// ```
    /// # use citro3d::math::*;
    /// # use std::f32::consts::PI;
    /// #
    /// # let _runner = test_runner::GdbRunner::default();
    ///
    /// let clip_planes = ClipPlanes {
    ///     near: 0.01,
    ///     far: 100.0,
    /// };
    ///
    /// let center = Matrix::perspective_projection(
    ///     PI / 4.0,
    ///     AspectRatio::BottomScreen,
    ///     ScreenOrientation::Rotated,
    ///     clip_planes,
    ///     CoordinateOrientation::LeftHanded,
    ///     None,
    /// );
    ///
    /// let right_eye = Matrix::perspective_projection(
    ///     PI / 4.0,
    ///     AspectRatio::BottomScreen,
    ///     ScreenOrientation::Rotated,
    ///     clip_planes,
    ///     CoordinateOrientation::LeftHanded,
    ///     Some(StereoDisplacement {
    ///         displacement: 1.0,
    ///         screen_depth: 2.0,
    ///     }),
    /// );
    /// ```
    #[doc(alias = "Mtx_Persp")]
    #[doc(alias = "Mtx_PerspTilt")]
    pub fn perspective_projection(
        vertical_fov_radians: f32,
        aspect_ratio: AspectRatio,
        orientation: ScreenOrientation,
        clip_plane: ClipPlanes,
        coordinates: CoordinateOrientation,
        stereo: Option<StereoDisplacement>,
    ) -> Self {
        let mut result = MaybeUninit::uninit();

        let left_handed = matches!(coordinates, CoordinateOrientation::LeftHanded);

        if let Some(stereo) = stereo {
            let initialize_mtx = orientation.perpsective_stereo_builder();
            unsafe {
                initialize_mtx(
                    result.as_mut_ptr(),
                    vertical_fov_radians,
                    aspect_ratio.into(),
                    clip_plane.near,
                    clip_plane.far,
                    stereo.displacement,
                    stereo.screen_depth,
                    left_handed,
                );
            }
        } else {
            let initialize_mtx = orientation.perspective_mono_builder();
            unsafe {
                initialize_mtx(
                    result.as_mut_ptr(),
                    vertical_fov_radians,
                    aspect_ratio.into(),
                    clip_plane.near,
                    clip_plane.far,
                    left_handed,
                );
            }
        }

        let inner = unsafe { result.assume_init() };

        Self(inner)
    }

    /// Helper function to build both eyes' perspective projection matrices
    /// at once. See [`perspective_projection`] for a description of each
    /// parameter.
    ///
    /// ```
    /// # use std::f32::consts::PI;
    /// # use citro3d::math::*;
    /// #
    /// # let _runner = test_runner::GdbRunner::default();
    ///
    /// let (left_eye, right_eye) = Matrix::stereo_projections(
    ///     PI / 4.0,
    ///     AspectRatio::TopScreen,
    ///     ScreenOrientation::Rotated,
    ///     ClipPlanes {
    ///         near: 0.01,
    ///         far: 100.0,
    ///     },
    ///     CoordinateOrientation::LeftHanded,
    ///     StereoDisplacement::new(0.5, 2.0),
    /// );
    /// ```
    ///
    /// [`perspective_projection`]: Self::perspective_projection
    #[doc(alias = "Mtx_PerspStereo")]
    #[doc(alias = "Mtx_PerspStereoTilt")]
    pub fn stereo_projections(
        vertical_fov_radians: f32,
        aspect_ratio: AspectRatio,
        orientation: ScreenOrientation,
        clip_plane: ClipPlanes,
        coordinates: CoordinateOrientation,
        (left_eye, right_eye): (StereoDisplacement, StereoDisplacement),
    ) -> (Self, Self) {
        let left = Self::perspective_projection(
            vertical_fov_radians,
            aspect_ratio,
            orientation,
            clip_plane,
            coordinates,
            Some(left_eye),
        );
        let right = Self::perspective_projection(
            vertical_fov_radians,
            aspect_ratio,
            orientation,
            clip_plane,
            coordinates,
            Some(right_eye),
        );
        (left, right)
    }

    pub(crate) fn as_raw(&self) -> *const citro3d_sys::C3D_Mtx {
        &self.0
    }
}

/// The [orientation](https://en.wikipedia.org/wiki/Orientation_(geometry))
/// (or "handedness") of the coordinate system.
#[derive(Clone, Copy, Debug)]
pub enum CoordinateOrientation {
    /// A left-handed coordinate system.
    LeftHanded,
    /// A right-handed coordinate system.
    RightHanded,
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

impl ScreenOrientation {
    fn perspective_mono_builder(
        self,
    ) -> unsafe extern "C" fn(*mut citro3d_sys::C3D_Mtx, f32, f32, f32, f32, bool) {
        match self {
            Self::Rotated => citro3d_sys::Mtx_PerspTilt,
            Self::None => citro3d_sys::Mtx_Persp,
        }
    }

    fn perpsective_stereo_builder(
        self,
    ) -> unsafe extern "C" fn(*mut citro3d_sys::C3D_Mtx, f32, f32, f32, f32, f32, f32, bool) {
        match self {
            Self::Rotated => citro3d_sys::Mtx_PerspStereoTilt,
            Self::None => citro3d_sys::Mtx_PerspStereo,
        }
    }

    // TODO: orthographic projections
    fn ortho_builder(
        self,
    ) -> unsafe extern "C" fn(*mut citro3d_sys::C3D_Mtx, f32, f32, f32, f32, f32, f32, bool) {
        match self {
            Self::Rotated => citro3d_sys::Mtx_OrthoTilt,
            Self::None => citro3d_sys::Mtx_Ortho,
        }
    }
}

/// Configuration for calculating stereoscopic projections.
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

/// Configuration for the [frustum](https://en.wikipedia.org/wiki/Viewing_frustum)
/// of a perspective projection.
#[derive(Clone, Copy, Debug)]
pub struct ClipPlanes {
    /// The z-depth of the near clip plane.
    pub near: f32,
    /// The z-depth of the far clip plane.
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
