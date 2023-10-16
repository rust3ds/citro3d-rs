use std::mem::MaybeUninit;

pub use private::Matrix;

use super::{CoordinateOrientation, FVec3};

mod private {
    use std::fmt;

    /// An `M`x`N` row-major matrix of `f32`s.
    #[doc(alias = "C3D_Mtx")]
    #[derive(Clone)]
    pub struct Matrix<const M: usize, const N: usize>(citro3d_sys::C3D_Mtx);

    impl<const M: usize, const N: usize> Matrix<M, N> {
        const ROW_SIZE: () = assert!(M == 3 || M == 4);
        const COLUMN_SIZE: () = assert!(N > 0 && N <= 4);

        // This constructor validates, at compile time, that the
        // constructed matrix is 3xN or 4xN matrix, where 0 < N ≤ 4.
        // We put this struct in a submodule to enforce that nothing creates
        // a Matrix without calling this constructor.
        #[allow(clippy::let_unit_value)]
        pub(crate) fn new(value: citro3d_sys::C3D_Mtx) -> Self {
            let () = Self::ROW_SIZE;
            let () = Self::COLUMN_SIZE;
            Self(value)
        }

        pub(crate) fn as_raw(&self) -> *const citro3d_sys::C3D_Mtx {
            &self.0
        }

        pub(crate) fn into_raw(self) -> citro3d_sys::C3D_Mtx {
            self.0
        }

        pub(crate) fn as_mut(&mut self) -> *mut citro3d_sys::C3D_Mtx {
            &mut self.0
        }
    }

    impl<const M: usize, const N: usize> fmt::Debug for Matrix<M, N> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let rows = unsafe { self.0.r }.map(|row| {
                // UNWRAP: N ≤ 4, so slicing to a smaller array should always work
                let mut row: [f32; N] = unsafe { row.c[..N].try_into() }.unwrap();
                // Rows are stored in WZYX order which is opposite of how most people
                // probably expect, so we reverse each row in-place as well.
                row.reverse();
                row
            });

            // UNWRAP: M ≤ 4, so slicing to a smaller array should always work
            let inner: [_; M] = rows[..M].try_into().unwrap();

            f.debug_tuple(std::any::type_name::<Self>())
                .field(&inner)
                .finish()
        }
    }
}

/// A 3x3 row-major matrix of `f32`s.
pub type Matrix3 = Matrix<3, 3>;
/// A 4x4 row-major matrix of `f32`s.
pub type Matrix4 = Matrix<4, 4>;

impl<const M: usize, const N: usize> Matrix<M, N> {
    /// Construct the zero matrix.
    #[doc(alias = "Mtx_Zeros")]
    pub fn zero() -> Self {
        // TODO: should this also be Default::default()?
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Zeros(out.as_mut_ptr());
            Self::new(out.assume_init())
        }
    }

    /// Transpose the matrix, swapping rows and columns.
    #[doc(alias = "Mtx_Transpose")]
    pub fn transpose(mut self) -> Matrix<N, M> {
        unsafe {
            citro3d_sys::Mtx_Transpose(self.as_mut());
        }
        Matrix::new(self.into_raw())
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
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        unsafe { citro3d_sys::Mtx_Translate(self.as_mut(), x, y, z, false) }
    }

    /// Scale a transformation matrix by the given amounts in the X, Y, and Z directions.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        unsafe { citro3d_sys::Mtx_Scale(self.as_mut(), x, y, z) }
    }

    /// Rotate a transformation matrix by the given angle around the given axis.
    pub fn rotate(&mut self, axis: FVec3, angle: f32) {
        unsafe { citro3d_sys::Mtx_Rotate(self.as_mut(), axis.0, angle, false) }
    }

    /// Rotate a transformation matrix by the given angle around the X axis.
    pub fn rotate_x(&mut self, angle: f32) {
        unsafe { citro3d_sys::Mtx_RotateX(self.as_mut(), angle, false) }
    }

    /// Rotate a transformation matrix by the given angle around the Y axis.
    pub fn rotate_y(&mut self, angle: f32) {
        unsafe { citro3d_sys::Mtx_RotateY(self.as_mut(), angle, false) }
    }

    /// Rotate a transformation matrix by the given angle around the Z axis.
    pub fn rotate_z(&mut self, angle: f32) {
        unsafe { citro3d_sys::Mtx_RotateZ(self.as_mut(), angle, false) }
    }

    // endregion
}

impl<const N: usize> Matrix<N, N> {
    /// Find the inverse of the matrix.
    ///
    /// # Errors
    ///
    /// If the matrix has no inverse, it will be returned unchanged as an [`Err`].
    #[doc(alias = "Mtx_Inverse")]
    pub fn inverse(mut self) -> Result<Self, Self> {
        let determinant = unsafe { citro3d_sys::Mtx_Inverse(self.as_mut()) };
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
            Self::new(out.assume_init())
        }
    }
}

impl Matrix3 {
    /// Construct a 3x3 matrix with the given values on the diagonal.
    #[doc(alias = "Mtx_Diagonal")]
    pub fn diagonal(x: f32, y: f32, z: f32) -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Diagonal(out.as_mut_ptr(), x, y, z, 0.0);
            Self::new(out.assume_init())
        }
    }
}

impl Matrix4 {
    /// Construct a 4x4 matrix with the given values on the diagonal.
    #[doc(alias = "Mtx_Diagonal")]
    pub fn diagonal(x: f32, y: f32, z: f32, w: f32) -> Self {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Diagonal(out.as_mut_ptr(), x, y, z, w);
            Self::new(out.assume_init())
        }
    }

    /// Construct a 3D transformation matrix for a camera, given its position,
    /// target, and upward direction.
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
            Self::new(out.assume_init())
        }
    }
}
