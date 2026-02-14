//! Floating-point vectors.

use std::fmt;

/// A vector of `f32`s.
///
/// # Layout
/// Note that this matches the PICA layout so is actually WZYX, this means using it
/// in vertex data as an attribute it will be reversed
///
/// It is guaranteed to have the same layout as [`citro3d_sys::C3D_FVec`] in memory
#[derive(Clone, Copy)]
#[doc(alias = "C3D_FVec")]
#[repr(transparent)]
pub struct FVec<const N: usize>(pub(crate) citro3d_sys::C3D_FVec);

/// A 3-vector of `f32`s.
pub type FVec3 = FVec<3>;

/// A 4-vector of `f32`s.
pub type FVec4 = FVec<4>;

impl<const N: usize> fmt::Debug for FVec<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = unsafe { self.0.__bindgen_anon_1 };
        let type_name = std::any::type_name::<Self>().split("::").last().unwrap();
        f.debug_tuple(type_name).field(&inner).finish()
    }
}

impl<const N: usize> FVec<N> {
    /// The vector's `x` component (also called the `i` component of `ijk[r]`).
    #[doc(alias = "i")]
    pub fn x(self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.x }
    }

    /// The vector's `y` component (also called the `j` component of `ijk[r]`).
    #[doc(alias = "j")]
    pub fn y(self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.y }
    }

    /// The vector's `i` component (also called the `k` component of `ijk[r]`).
    #[doc(alias = "k")]
    pub fn z(self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.z }
    }
}

impl FVec4 {
    /// The vector's `w` component (also called `r` for the real component of `ijk[r]`).
    #[doc(alias = "r")]
    pub fn w(self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.w }
    }

    /// Wrap a raw [`citro3d_sys::C3D_FVec`]
    pub fn from_raw(raw: citro3d_sys::C3D_FVec) -> Self {
        Self(raw)
    }

    /// Create a new [`FVec4`] from its components.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// let v = FVec4::new(1.0, 2.0, 3.0, 4.0);
    /// ```
    #[doc(alias = "FVec4_New")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(unsafe { citro3d_sys::FVec4_New(x, y, z, w) })
    }

    /// Create a new [`FVec4`], setting each component to `v`.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use approx::assert_abs_diff_eq;
    /// let v = FVec4::splat(1.0);
    /// assert_abs_diff_eq!(v, FVec4::new(1.0, 1.0, 1.0, 1.0));
    /// ```
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    /// Divide the vector's XYZ components by its W component.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use approx::assert_abs_diff_eq;
    /// let v = FVec4::new(2.0, 4.0, 6.0, 2.0);
    /// assert_abs_diff_eq!(v.perspective_divide(), FVec4::new(1.0, 2.0, 3.0, 1.0));
    /// ```
    #[doc(alias = "FVec4_PerspDivide")]
    pub fn perspective_divide(self) -> Self {
        Self(unsafe { citro3d_sys::FVec4_PerspDivide(self.0) })
    }

    /// The dot product of two vectors.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use approx::assert_abs_diff_eq;
    /// let v1 = FVec4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = FVec4::new(1.0, 0.5, 1.0, 0.5);
    /// assert_abs_diff_eq!(v1.dot(v2), 7.0);
    /// ```
    #[doc(alias = "FVec4_Dot")]
    pub fn dot(self, rhs: Self) -> f32 {
        unsafe { citro3d_sys::FVec4_Dot(self.0, rhs.0) }
    }

    /// The magnitude of the vector.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use approx::assert_abs_diff_eq;
    /// let v = FVec4::splat(1.0);
    /// assert_abs_diff_eq!(v.magnitude(), 2.0);
    /// ```
    #[doc(alias = "FVec4_Magnitude")]
    pub fn magnitude(self) -> f32 {
        unsafe { citro3d_sys::FVec4_Magnitude(self.0) }
    }

    /// Normalize the vector to a magnitude of `1.0`.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use approx::assert_abs_diff_eq;
    /// let v = FVec4::new(1.0, 2.0, 2.0, 4.0);
    /// assert_abs_diff_eq!(v.normalize(), FVec4::new(0.2, 0.4, 0.4, 0.8));
    /// ```
    #[doc(alias = "FVec4_Normalize")]
    pub fn normalize(self) -> Self {
        Self(unsafe { citro3d_sys::FVec4_Normalize(self.0) })
    }
}

impl Into<[f32; 4]> for FVec4 {
    fn into(self) -> [f32; 4] {
        [self.x(), self.y(), self.z(), self.w()]
    }
}

impl FVec3 {
    /// Create a new [`FVec3`] from its components.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// let v = FVec3::new(1.0, 2.0, 3.0);
    /// ```
    #[doc(alias = "FVec3_New")]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(unsafe { citro3d_sys::FVec3_New(x, y, z) })
    }

    /// Create a new [`FVec3`], setting each component to the given `v`.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// let v = FVec3::splat(1.0);
    /// ```
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// The distance between two points in 3D space.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// # use approx::assert_abs_diff_eq;
    /// let l = FVec3::new(1.0, 3.0, 4.0);
    /// let r = FVec3::new(0.0, 1.0, 2.0);
    ///
    /// assert_abs_diff_eq!(l.distance(r), 3.0);
    /// ```
    #[doc(alias = "FVec3_Distance")]
    pub fn distance(self, rhs: Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Distance(self.0, rhs.0) }
    }

    /// The cross product of two 3D vectors.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// # use approx::assert_abs_diff_eq;
    /// let l = FVec3::new(1.0, 0.0, 0.0);
    /// let r = FVec3::new(0.0, 1.0, 0.0);
    /// assert_abs_diff_eq!(l.cross(r), FVec3::new(0.0, 0.0, 1.0));
    /// ```
    #[doc(alias = "FVec3_Cross")]
    pub fn cross(self, rhs: Self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Cross(self.0, rhs.0) })
    }

    /// The dot product of two vectors.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// # use approx::assert_abs_diff_eq;
    /// let l = FVec3::new(1.0, 2.0, 3.0);
    /// let r = FVec3::new(3.0, 2.0, 1.0);
    /// assert_abs_diff_eq!(l.dot(r), 10.0);
    /// ```
    #[doc(alias = "FVec3_Dot")]
    pub fn dot(self, rhs: Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Dot(self.0, rhs.0) }
    }

    /// The magnitude of the vector.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// # use approx::assert_abs_diff_eq;
    /// let v = FVec3::splat(3.0f32.sqrt());
    /// assert_abs_diff_eq!(v.magnitude(), 3.0);
    /// ```
    #[doc(alias = "FVec3_Magnitude")]
    pub fn magnitude(self) -> f32 {
        unsafe { citro3d_sys::FVec3_Magnitude(self.0) }
    }

    /// Normalize the vector to a magnitude of `1.0`.
    ///
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec3;
    /// # use approx::assert_abs_diff_eq;
    /// let v = FVec3::splat(1.0);
    /// assert_abs_diff_eq!(v.normalize(), FVec3::splat(1.0 / 3.0_f32.sqrt()));
    /// ```
    #[doc(alias = "FVec3_Normalize")]
    pub fn normalize(self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Normalize(self.0) })
    }
}

#[cfg(feature = "glam")]
impl From<glam::Vec4> for FVec4 {
    fn from(value: glam::Vec4) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}
#[cfg(feature = "glam")]
impl From<glam::Vec3> for FVec3 {
    fn from(value: glam::Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}
#[cfg(feature = "glam")]
impl From<FVec4> for glam::Vec4 {
    fn from(value: FVec4) -> Self {
        glam::Vec4::new(value.x(), value.y(), value.z(), value.w())
    }
}

#[cfg(feature = "glam")]
impl From<FVec3> for glam::Vec3 {
    fn from(value: FVec3) -> Self {
        glam::Vec3::new(value.x(), value.y(), value.z())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn fvec4() {
        let v = FVec4::new(1.0, 2.0, 3.0, 4.0);
        let actual = [v.x(), v.y(), v.z(), v.w()];
        let expected = [1.0, 2.0, 3.0, 4.0];
        assert_abs_diff_eq!(&actual[..], &expected[..]);
    }

    #[test]
    fn fvec3() {
        let v = FVec3::new(1.0, 2.0, 3.0);
        let actual = [v.x(), v.y(), v.z()];
        let expected = [1.0, 2.0, 3.0];
        assert_abs_diff_eq!(&actual[..], &expected[..]);
    }
}
