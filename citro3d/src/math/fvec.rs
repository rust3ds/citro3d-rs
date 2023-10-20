//! Floating-point vectors.

use std::fmt;

/// A vector of `f32`s.
#[derive(Clone, Copy)]
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

impl FVec4 {
    /// Create a new [`FVec4`] from its components.
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
    /// # use float_cmp::assert_approx_eq;
    /// let v = FVec4::splat(1.0);
    /// assert_approx_eq!(FVec4, v, FVec4::new(1.0, 1.0, 1.0, 1.0));
    /// ```
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    /// The vector's `w` component (sometimes also called `r` for the real
    /// component of a quaternion `ijk[r]`).
    #[doc(alias = "r")]
    pub fn w(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.w }
    }

    /// Divide the vector's XYZ components by its W component.
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use float_cmp::assert_approx_eq;
    /// let v = FVec4::new(2.0, 4.0, 6.0, 2.0);
    /// assert_approx_eq!(
    ///     FVec4,
    ///     v.perspective_divide(),
    ///     FVec4::new(1.0, 2.0, 3.0, 1.0)
    /// );
    /// ```
    #[doc(alias = "FVec4_PerspDivide")]
    pub fn perspective_divide(&self) -> Self {
        Self(unsafe { citro3d_sys::FVec4_PerspDivide(self.0) })
    }

    /// The dot product of two vectors.
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use float_cmp::assert_approx_eq;
    /// let v1 = FVec4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = FVec4::new(1.0, 0.5, 1.0, 0.5);
    /// assert_approx_eq!(f32, v1.dot(&v2), 7.0);
    /// ```
    #[doc(alias = "FVec4_Dot")]
    pub fn dot(&self, rhs: &Self) -> f32 {
        unsafe { citro3d_sys::FVec4_Dot(self.0, rhs.0) }
    }

    /// The magnitude of the vector.
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use float_cmp::assert_approx_eq;
    /// let v = FVec4::splat(1.0);
    /// assert_approx_eq!(f32, v.magnitude(), 2.0);
    /// ```
    #[doc(alias = "FVec4_Magnitude")]
    pub fn magnitude(&self) -> f32 {
        unsafe { citro3d_sys::FVec4_Magnitude(self.0) }
    }

    /// Normalize the vector to a magnitude of `1.0`.
    /// # Example
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::math::FVec4;
    /// # use float_cmp::assert_approx_eq;
    /// let v = FVec4::new(1.0, 2.0, 2.0, 4.0);
    /// assert_approx_eq!(FVec4, v, FVec4::new(0.1, 0.4, 0.4, 0.8));
    /// ```
    #[doc(alias = "FVec3_Normalize")]
    pub fn normalize(&self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Normalize(self.0) })
    }
}

impl FVec3 {
    /// Create a new [`FVec3`] from its components.
    #[doc(alias = "FVec3_New")]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(unsafe { citro3d_sys::FVec3_New(x, y, z) })
    }

    /// Create a new [`FVec3`], setting each component to the given `v`.
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// The distance between two points in 3D space.
    #[doc(alias = "FVec3_Distance")]
    pub fn distance(&self, rhs: &Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Distance(self.0, rhs.0) }
    }

    /// The cross product of two 3D vectors.
    #[doc(alias = "FVec3_Cross")]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Cross(self.0, rhs.0) })
    }

    /// The dot product of two vectors.
    #[doc(alias = "FVec3_Dot")]
    pub fn dot(&self, rhs: &Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Dot(self.0, rhs.0) }
    }

    /// The magnitude of the vector.
    #[doc(alias = "FVec3_Magnitude")]
    pub fn magnitude(&self) -> f32 {
        unsafe { citro3d_sys::FVec3_Magnitude(self.0) }
    }

    /// Normalize the vector to a magnitude of `1.0`.
    #[doc(alias = "FVec3_Normalize")]
    pub fn normalize(&self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Normalize(self.0) })
    }
}

impl<const N: usize> FVec<N> {
    /// The vector's `x` component (sometimes also called the `i` component of `ijk[r]`).
    pub fn x(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.x }
    }

    /// The vector's `y` component (sometimes also called the `j` component of `ijk[r]`).
    pub fn y(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.y }
    }

    /// The vector's `i` component (sometimes also called the `k` component of `ijk[r]`).
    pub fn z(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.z }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn fvec4() {
        let v = FVec4::new(1.0, 2.0, 3.0, 4.0);
        let actual = [v.x(), v.y(), v.z(), v.w()];
        let expected = [1.0, 2.0, 3.0, 4.0];
        assert_approx_eq!(&[f32], &actual, &expected);
    }

    #[test]
    fn fvec3() {
        let v = FVec3::new(1.0, 2.0, 3.0);
        let actual = [v.x(), v.y(), v.z()];
        let expected = [1.0, 2.0, 3.0];
        assert_approx_eq!(&[f32], &actual, &expected);

        let l = FVec3::new(2.0, 2.0, 2.0);

        assert_eq!(l, FVec3::splat(2.0));

        for component in [l.x(), l.y(), l.z()] {
            assert_approx_eq!(f32, component, 2.0);
        }

        let dot = l.dot(&FVec3::splat(3.0));
        assert_approx_eq!(f32, dot, 18.0);

        assert_approx_eq!(f32, l.magnitude(), f32::sqrt(12.0));

        let norm = l.normalize();
        assert_approx_eq!(f32, norm.magnitude(), 1.0);
        for component in [l.y(), l.z()] {
            assert_approx_eq!(f32, l.x(), component);
        }
    }
}
