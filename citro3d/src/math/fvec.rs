//! Floating-point vectors.

use std::fmt;

/// A vector of `f32`s.
#[derive(Clone, Copy)]
pub struct FVec<const N: usize>(pub(super) citro3d_sys::C3D_FVec);

/// A 3-vector of `f32`s.
pub type FVec3 = FVec<3>;

/// A 4-vector of `f32`s.
pub type FVec4 = FVec<4>;

impl<const N: usize> fmt::Debug for FVec<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = unsafe { self.0.c };
        f.debug_tuple(std::any::type_name::<Self>())
            .field(&inner)
            .finish()
    }
}

impl<Rhs, const N: usize> PartialEq<Rhs> for FVec<N>
where
    Rhs: Copy,
    Self: From<Rhs>,
{
    fn eq(&self, other: &Rhs) -> bool {
        unsafe { self.0.c == Self::from(*other).0.c }
    }
}

impl<const N: usize> Eq for FVec<N> {}

impl FVec4 {
    /// Create a new [`FVec4`] from its components.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(unsafe { citro3d_sys::FVec4_New(x, y, z, w) })
    }

    /// Create a new [`FVec4`], setting each component to `v`.
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    /// The vector's `w` component (sometimes also called the `r` component of `ijk[r]`).
    #[doc(alias = "r")]
    pub fn w(&self) -> f32 {
        unsafe { self.0.__bindgen_anon_1.w }
    }

    /// Divide the vector's XYZ components by its W component.
    pub fn perspective_divide(&self) -> Self {
        Self(unsafe { citro3d_sys::FVec4_PerspDivide(self.0) })
    }

    /// The dot product of two vectors.
    pub fn dot(&self, rhs: &Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Dot(self.0, rhs.0) }
    }

    /// The magnitude of the vector.
    pub fn magnitude(&self) -> f32 {
        unsafe { citro3d_sys::FVec3_Magnitude(self.0) }
    }

    /// Normalize the vector to a magnitude of `1.0`.
    pub fn normalize(&self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Normalize(self.0) })
    }
}

impl FVec3 {
    /// Create a new [`FVec3`] from its components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(unsafe { citro3d_sys::FVec3_New(x, y, z) })
    }

    /// Create a new [`FVec3`], setting each component to the given `v`.
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// The distance between two points in 3D space.
    pub fn distance(&self, rhs: &Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Distance(self.0, rhs.0) }
    }

    /// The cross product of two 3D vectors.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self(unsafe { citro3d_sys::FVec3_Cross(self.0, rhs.0) })
    }

    /// The dot product of two vectors.
    pub fn dot(&self, rhs: &Self) -> f32 {
        unsafe { citro3d_sys::FVec3_Dot(self.0, rhs.0) }
    }

    /// The magnitude of the vector.
    pub fn magnitude(&self) -> f32 {
        unsafe { citro3d_sys::FVec3_Magnitude(self.0) }
    }

    /// Normalize the vector to a magnitude of `1.0`.
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
    use super::*;

    #[test]
    fn fvec4() {
        let l = FVec4::new(2.0, 2.0, 2.0, 2.0);

        assert_eq!(l, FVec4::splat(2.0));

        for component in [l.x(), l.y(), l.z(), l.w()] {
            assert!((component - 2.0).abs() < f32::EPSILON);
        }

        assert_eq!(l.perspective_divide(), FVec4::splat(1.0));

        let dot = l.dot(&FVec4::splat(3.0));
        assert!((dot - 24.0).abs() < f32::EPSILON);

        assert!((l.magnitude() - 8.0).abs() < f32::EPSILON);

        let norm = l.normalize();
        assert!((norm.magnitude() - 1.0).abs() < f32::EPSILON);
        for component in [l.y(), l.z(), l.w()] {
            assert!((component - l.x()).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn fvec3() {
        let l = FVec3::new(2.0, 2.0, 2.0);

        assert_eq!(l, FVec3::splat(2.0));

        for component in [l.x(), l.y(), l.z()] {
            assert!((component - 2.0).abs() < f32::EPSILON);
        }

        let dot = l.dot(&FVec3::splat(3.0));
        assert!((dot - 18.0).abs() < f32::EPSILON);

        assert!((l.magnitude() - 8.0).abs() < f32::EPSILON);

        let norm = l.normalize();
        assert!((norm.magnitude() - 1.0).abs() < f32::EPSILON);
        for component in [l.y(), l.z()] {
            assert!((l.x() - component).abs() < f32::EPSILON);
        }
    }
}
