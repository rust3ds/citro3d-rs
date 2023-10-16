use std::mem::MaybeUninit;
use std::ops::Range;

use super::{
    AspectRatio, ClipPlanes, CoordinateOrientation, Matrix4, ScreenOrientation, StereoDisplacement,
};

/// Configuration for a 3D [projection](https://en.wikipedia.org/wiki/3D_projection).
/// See specific `Kind` implementations for constructors, e.g.
/// [`Projection::perspective`] and [`Projection::orthographic`].
///
/// To use the resulting projection, convert it to a [`Matrix`](super::Matrix) with [`From`]/[`Into`].
#[derive(Clone, Debug)]
pub struct Projection<Kind> {
    coordinates: CoordinateOrientation,
    rotation: ScreenOrientation,
    inner: Kind,
}

impl<Kind> Projection<Kind> {
    fn new(inner: Kind) -> Self {
        Self {
            coordinates: CoordinateOrientation::default(),
            rotation: ScreenOrientation::default(),
            inner,
        }
    }

    /// Set the coordinate system's orientation for the projection.
    /// See [`CoordinateOrientation`] for more details.
    pub fn coordinates(&mut self, orientation: CoordinateOrientation) -> &mut Self {
        self.coordinates = orientation;
        self
    }

    /// Set the screen rotation for the projection.
    /// See [`ScreenOrientation`] for more details.
    pub fn screen(&mut self, orientation: ScreenOrientation) -> &mut Self {
        self.rotation = orientation;
        self
    }
}

/// See [`Projection::perspective`].
#[derive(Clone, Debug)]
pub struct Perspective {
    vertical_fov_radians: f32,
    aspect_ratio: AspectRatio,
    clip_planes: ClipPlanes,
    stereo: Option<StereoDisplacement>,
}

impl Projection<Perspective> {
    /// Construct a projection matrix suitable for projecting 3D world space onto
    /// the 3DS screens.
    ///
    /// # Parameters
    ///
    /// * `vertical_fov`: the vertical field of view, measured in radians
    /// * `aspect_ratio`: the aspect ratio of the projection
    /// * `clip_planes`: the near and far clip planes of the view frustum.
    ///   [`ClipPlanes`] are always defined by near and far values, regardless
    ///   of the projection's [`CoordinateOrientation`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use citro3d::math::*;
    /// # use std::f32::consts::PI;
    /// #
    /// # let _runner = test_runner::GdbRunner::default();
    /// #
    /// let clip_planes = ClipPlanes {
    ///     near: 0.01,
    ///     far: 100.0,
    /// };
    ///
    /// let bottom: Matrix =
    ///     Projection::perspective(PI / 4.0, AspectRatio::BottomScreen, clip_planes).into();
    ///
    /// let top: Matrix = Projection::perspective(PI / 4.0, AspectRatio::TopScreen, clip_planes).into();
    /// ```
    #[doc(alias = "Mtx_Persp")]
    #[doc(alias = "Mtx_PerspTilt")]
    pub fn perspective(
        vertical_fov_radians: f32,
        aspect_ratio: AspectRatio,
        clip_planes: ClipPlanes,
    ) -> Self {
        Self::new(Perspective {
            vertical_fov_radians,
            aspect_ratio,
            clip_planes,
            stereo: None,
        })
    }

    /// Helper function to build both eyes' perspective projection matrices
    /// at once. See [`StereoDisplacement`] for details on how to configure
    /// stereoscopy.
    ///
    /// ```
    /// # use std::f32::consts::PI;
    /// # use citro3d::math::*;
    /// #
    /// # let _runner = test_runner::GdbRunner::default();
    /// #
    /// let (left, right) = StereoDisplacement::new(0.5, 2.0);
    /// let (left_eye, right_eye) = Projection::perspective(
    ///     PI / 4.0,
    ///     AspectRatio::TopScreen,
    ///     ClipPlanes {
    ///         near: 0.01,
    ///         far: 100.0,
    ///     },
    /// )
    /// .stereo_matrices(left, right);
    /// ```
    #[doc(alias = "Mtx_PerspStereo")]
    #[doc(alias = "Mtx_PerspStereoTilt")]
    pub fn stereo_matrices(
        self,
        left_eye: StereoDisplacement,
        right_eye: StereoDisplacement,
    ) -> (Matrix4, Matrix4) {
        // TODO: we might be able to avoid this clone if there was a conversion
        // from &Self to Matrix instead of Self... but it's probably fine for now
        let left = self.clone().stereo(left_eye);
        let right = self.stereo(right_eye);
        // Also, we could consider just returning (Self, Self) here? idk
        (left.into(), right.into())
    }

    fn stereo(mut self, displacement: StereoDisplacement) -> Self {
        self.inner.stereo = Some(displacement);
        self
    }
}

impl From<Projection<Perspective>> for Matrix4 {
    fn from(projection: Projection<Perspective>) -> Self {
        let Perspective {
            vertical_fov_radians,
            aspect_ratio,
            clip_planes,
            stereo,
        } = projection.inner;

        let mut result = MaybeUninit::uninit();

        if let Some(stereo) = stereo {
            let make_mtx = match projection.rotation {
                ScreenOrientation::Rotated => citro3d_sys::Mtx_PerspStereoTilt,
                ScreenOrientation::None => citro3d_sys::Mtx_PerspStereo,
            };
            unsafe {
                make_mtx(
                    result.as_mut_ptr(),
                    vertical_fov_radians,
                    aspect_ratio.into(),
                    clip_planes.near,
                    clip_planes.far,
                    stereo.displacement,
                    stereo.screen_depth,
                    projection.coordinates.is_left_handed(),
                );
            }
        } else {
            let make_mtx = match projection.rotation {
                ScreenOrientation::Rotated => citro3d_sys::Mtx_PerspTilt,
                ScreenOrientation::None => citro3d_sys::Mtx_Persp,
            };
            unsafe {
                make_mtx(
                    result.as_mut_ptr(),
                    vertical_fov_radians,
                    aspect_ratio.into(),
                    clip_planes.near,
                    clip_planes.far,
                    projection.coordinates.is_left_handed(),
                );
            }
        }

        unsafe { Self::new(result.assume_init()) }
    }
}

/// See [`Projection::orthographic`].
#[derive(Clone, Debug)]
pub struct Orthographic {
    clip_planes_x: Range<f32>,
    clip_planes_y: Range<f32>,
    clip_planes_z: ClipPlanes,
}

impl Projection<Orthographic> {
    /// Construct an orthographic projection. The X and Y clip planes are passed
    /// as ranges because their coordinates are always oriented the same way
    /// (+X right, +Y up).
    ///
    /// The Z [`ClipPlanes`], however, are always defined by
    /// near and far values, regardless of the projection's [`CoordinateOrientation`].
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::{Projection, ClipPlanes, Matrix};
    /// #
    /// let mtx: Matrix = Projection::orthographic(
    ///     0.0..240.0,
    ///     0.0..400.0,
    ///     ClipPlanes {
    ///         near: 0.0,
    ///         far: 100.0,
    ///     },
    /// )
    /// .into();
    /// ```
    #[doc(alias = "Mtx_Ortho")]
    #[doc(alias = "Mtx_OrthoTilt")]
    pub fn orthographic(
        clip_planes_x: Range<f32>,
        clip_planes_y: Range<f32>,
        clip_planes_z: ClipPlanes,
    ) -> Self {
        Self::new(Orthographic {
            clip_planes_x,
            clip_planes_y,
            clip_planes_z,
        })
    }
}

impl From<Projection<Orthographic>> for Matrix4 {
    fn from(projection: Projection<Orthographic>) -> Self {
        let make_mtx = match projection.rotation {
            ScreenOrientation::Rotated => citro3d_sys::Mtx_OrthoTilt,
            ScreenOrientation::None => citro3d_sys::Mtx_Ortho,
        };

        let Orthographic {
            clip_planes_x,
            clip_planes_y,
            clip_planes_z,
        } = projection.inner;

        let mut out = MaybeUninit::uninit();
        unsafe {
            make_mtx(
                out.as_mut_ptr(),
                clip_planes_x.start,
                clip_planes_x.end,
                clip_planes_y.start,
                clip_planes_y.end,
                clip_planes_z.near,
                clip_planes_z.far,
                projection.coordinates.is_left_handed(),
            );
            Self::new(out.assume_init())
        }
    }
}
