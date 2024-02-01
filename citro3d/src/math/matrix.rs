use std::mem::MaybeUninit;

use super::{CoordinateOrientation, FVec3, FVec4};

/// A 4x4 row-major matrix of `f32`s.
///
/// # Layout details
/// Rows are actually stored as WZYX in memory. There are helper functions
/// for accessing the rows in XYZW form. The `Debug` implementation prints
/// the shows in WZYX form
///
/// It is also guaranteed to have the same layout as [`citro3d_sys::C3D_Mtx`]
#[doc(alias = "C3D_Mtx")]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Matrix4(citro3d_sys::C3D_Mtx);

impl Matrix4 {
    /// Create a new matrix from a raw citro3d_sys one
    pub fn from_raw(value: citro3d_sys::C3D_Mtx) -> Self {
        Self(value)
    }

    pub fn as_raw(&self) -> &citro3d_sys::C3D_Mtx {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut citro3d_sys::C3D_Mtx {
        &mut self.0
    }

    pub fn into_raw(self) -> citro3d_sys::C3D_Mtx {
        self.0
    }

    /// Get the rows in raw (WZYX) form
    pub fn rows_wzyx(self) -> [FVec4; 4] {
        // Safety: FVec4 is repr(C) to allow transmute from C3D_Vec
        unsafe { core::mem::transmute::<[citro3d_sys::C3D_FVec; 4], [FVec4; 4]>(self.0.r) }
    }

    /// Get the rows in normal XYZW form
    pub fn rows_xyzw(self) -> [[f32; 4]; 4] {
        let mut rows = self.rows_wzyx();
        for r in &mut rows {
            unsafe {
                r.0.c.reverse();
            }
        }
        // Safety: FVec has same layout as citro3d_sys version which is a union with [f32; 4] as one variant
        unsafe { std::mem::transmute::<_, [[f32; 4]; 4]>(rows) }
    }
    /// Construct the zero matrix.
    #[doc(alias = "Mtx_Zeros")]
    pub fn zero() -> Self {
        // TODO: should this also be Default::default()?
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Zeros(out.as_mut_ptr());
            Self::from_raw(out.assume_init())
        }
    }

    /// Transpose the matrix, swapping rows and columns.
    #[doc(alias = "Mtx_Transpose")]
    pub fn transpose(mut self) -> Matrix4 {
        unsafe {
            citro3d_sys::Mtx_Transpose(self.as_raw_mut());
        }
        Matrix4::from_raw(self.into_raw())
    }

    // region: Matrix transformations
    //
    // NOTE: the `bRightSide` arg common to many of these APIs flips the order of
    // operations so that a transformation occurs as self(T) instead of T(self).
    // For now I'm not sure if that's a common use case, but if needed we could
    // probably have some kinda wrapper type that does transformations in the
    // opposite order, or an enum arg for these APIs or something.

    /// Translate a transformation matrix by the given amounts in the X, Y, and Z
    /// directions.
    #[doc(alias = "Mtx_Translate")]
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        unsafe { citro3d_sys::Mtx_Translate(self.as_raw_mut(), x, y, z, false) }
    }

    /// Scale a transformation matrix by the given amounts in the X, Y, and Z directions.
    #[doc(alias = "Mtx_Scale")]
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        unsafe { citro3d_sys::Mtx_Scale(self.as_raw_mut(), x, y, z) }
    }

    /// Rotate a transformation matrix by the given angle around the given axis.
    #[doc(alias = "Mtx_Rotate")]
    pub fn rotate(&mut self, axis: FVec3, angle: f32) {
        unsafe { citro3d_sys::Mtx_Rotate(self.as_raw_mut(), axis.0, angle, false) }
    }

    /// Rotate a transformation matrix by the given angle around the X axis.
    #[doc(alias = "Mtx_RotateX")]
    pub fn rotate_x(&mut self, angle: f32) {
        unsafe { citro3d_sys::Mtx_RotateX(self.as_raw_mut(), angle, false) }
    }

    /// Rotate a transformation matrix by the given angle around the Y axis.
    #[doc(alias = "Mtx_RotateY")]
    pub fn rotate_y(&mut self, angle: f32) {
        unsafe { citro3d_sys::Mtx_RotateY(self.as_raw_mut(), angle, false) }
    }

    /// Rotate a transformation matrix by the given angle around the Z axis.
    #[doc(alias = "Mtx_RotateZ")]
    pub fn rotate_z(&mut self, angle: f32) {
        unsafe { citro3d_sys::Mtx_RotateZ(self.as_raw_mut(), angle, false) }
    }

    /// Find the inverse of the matrix.
    ///
    /// # Errors
    ///
    /// If the matrix has no inverse, it will be returned unchanged as an [`Err`].
    #[doc(alias = "Mtx_Inverse")]
    pub fn inverse(mut self) -> Result<Self, Self> {
        let determinant = unsafe { citro3d_sys::Mtx_Inverse(self.as_raw_mut()) };
        if determinant == 0.0 {
            Err(self)
        } else {
            Ok(self)
        }
    }

    /// Construct the identity matrix.
    #[doc(alias = "Mtx_Identity")]
    pub fn identity() -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Identity(out.as_mut_ptr());
            Self::from_raw(out.assume_init())
        }
    }

    /// Construct a 4x4 matrix with the given values on the diagonal.
    #[doc(alias = "Mtx_Diagonal")]
    pub fn diagonal(x: f32, y: f32, z: f32, w: f32) -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Diagonal(out.as_mut_ptr(), x, y, z, w);
            Self::from_raw(out.assume_init())
        }
    }

    /// Construct a 3D transformation matrix for a camera, given its position,
    /// target, and upward direction.
    #[doc(alias = "Mtx_LookAt")]
    pub fn looking_at(
        camera_position: FVec3,
        camera_target: FVec3,
        camera_up: FVec3,
        coordinates: CoordinateOrientation,
    ) -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_LookAt(
                out.as_mut_ptr(),
                camera_position.0,
                camera_target.0,
                camera_up.0,
                coordinates.is_left_handed(),
            );
            Self::from_raw(out.assume_init())
        }
    }
}

impl core::fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Matrix4").field(&self.rows_wzyx()).finish()
    }
}
impl PartialEq<Matrix4> for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        self.rows_wzyx() == other.rows_wzyx()
    }
}
impl Eq for Matrix4 {}
