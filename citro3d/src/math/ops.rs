use std::mem::MaybeUninit;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "approx")]
use approx::AbsDiffEq;

use super::{FVec, FVec3, FVec4, Matrix4};

// region: FVec4 math operators

impl Add for FVec4 {
    type Output = Self;

    #[doc(alias = "FVec4_Add")]
    fn add(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Add(self.0, rhs.0) })
    }
}

impl Sub for FVec4 {
    type Output = Self;

    #[doc(alias = "FVec4_Subtract")]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Subtract(self.0, rhs.0) })
    }
}

impl Neg for FVec4 {
    type Output = Self;

    #[doc(alias = "FVec4_Negate")]
    fn neg(self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Negate(self.0) })
    }
}

impl Mul<f32> for FVec4 {
    type Output = Self;

    #[doc(alias = "FVec4_Scale")]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Scale(self.0, rhs) })
    }
}

// endregion

// region: FVec3 math operators

impl Add for FVec3 {
    type Output = Self;

    #[doc(alias = "FVec3_Add")]
    fn add(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Add(self.0, rhs.0) })
    }
}

impl Sub for FVec3 {
    type Output = Self;

    #[doc(alias = "FVec3_Subtract")]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Subtract(self.0, rhs.0) })
    }
}

impl Neg for FVec3 {
    type Output = Self;

    #[doc(alias = "FVec3_Negate")]
    fn neg(self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Negate(self.0) })
    }
}

impl Mul<f32> for FVec3 {
    type Output = Self;

    #[doc(alias = "FVec3_Scale")]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Scale(self.0, rhs) })
    }
}

// endregion

impl<const N: usize> Div<f32> for FVec<N>
where
    FVec<N>: Mul<f32>,
{
    type Output = <Self as Mul<f32>>::Output;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<const N: usize> PartialEq for FVec<N> {
    fn eq(&self, other: &Self) -> bool {
        let range = (4 - N)..;
        unsafe { self.0.c[range.clone()] == other.0.c[range] }
    }
}

impl<const N: usize> Eq for FVec<N> {}

#[cfg(feature = "approx")]
impl<const N: usize> AbsDiffEq for FVec<N> {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        // See https://docs.rs/almost/latest/almost/#why-another-crate
        // for rationale of using this over just EPSILON
        f32::EPSILON.sqrt()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let range = (4 - N)..;
        let (lhs, rhs) = unsafe { (&self.0.c[range.clone()], &other.0.c[range]) };
        lhs.abs_diff_eq(rhs, epsilon)
    }
}

// region: Matrix math operators

impl Add<Matrix4> for Matrix4 {
    type Output = Matrix4;

    #[doc(alias = "Mtx_Add")]
    fn add(self, rhs: Matrix4) -> Self::Output {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Add(out.as_mut_ptr(), self.as_raw(), rhs.as_raw());
            Matrix4::from_raw(out.assume_init())
        }
    }
}

impl Sub<Matrix4> for Matrix4 {
    type Output = Matrix4;

    #[doc(alias = "Mtx_Subtract")]
    fn sub(self, rhs: Matrix4) -> Self::Output {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Subtract(out.as_mut_ptr(), self.as_raw(), rhs.as_raw());
            Matrix4::from_raw(out.assume_init())
        }
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    #[doc(alias = "Mtx_Multiply")]
    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Multiply(out.as_mut_ptr(), self.as_raw(), rhs.as_raw());
            Matrix4::from_raw(out.assume_init())
        }
    }
}

impl Mul<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        *self * rhs
    }
}

impl Mul<FVec4> for &Matrix4 {
    type Output = FVec4;

    #[doc(alias = "Mtx_MultiplyFVec4")]
    fn mul(self, rhs: FVec4) -> Self::Output {
        FVec(unsafe { citro3d_sys::Mtx_MultiplyFVec4(self.as_raw(), rhs.0) })
    }
}

impl Mul<FVec3> for &Matrix4 {
    type Output = FVec4;

    #[doc(alias = "Mtx_MultiplyFVecH")]
    fn mul(self, rhs: FVec3) -> Self::Output {
        FVec(unsafe { citro3d_sys::Mtx_MultiplyFVecH(self.as_raw(), rhs.0) })
    }
}

impl PartialEq<Matrix4> for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        self.rows_wzyx() == other.rows_wzyx()
    }
}

// endregion

#[cfg(feature = "approx")]
#[doc(cfg(feature = "approx"))]
impl AbsDiffEq for Matrix4 {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        // See https://docs.rs/almost/latest/almost/#why-another-crate
        // for rationale of using this over just EPSILON
        f32::EPSILON.sqrt()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.rows_wzyx()
            .into_iter()
            .zip(other.rows_wzyx())
            .all(|(l, r)| l.abs_diff_eq(&r, epsilon))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn fvec3() {
        let l = FVec3::splat(1.0);
        let r = FVec3::splat(2.0);

        assert_abs_diff_eq!(l + r, FVec3::splat(3.0));
        assert_abs_diff_eq!(l - r, FVec3::splat(-1.0));
        assert_abs_diff_eq!(-l, FVec3::splat(-1.0));
        assert_abs_diff_eq!(l * 1.5, FVec3::splat(1.5));
        assert_abs_diff_eq!(l / 2.0, FVec3::splat(0.5));
    }

    #[test]
    fn fvec4() {
        let l = FVec4::splat(1.0);
        let r = FVec4::splat(2.0);

        assert_abs_diff_eq!(l + r, FVec4::splat(3.0));
        assert_abs_diff_eq!(l - r, FVec4::splat(-1.0));
        assert_abs_diff_eq!(-l, FVec4::splat(-1.0));
        assert_abs_diff_eq!(l * 1.5, FVec4::splat(1.5));
        assert_abs_diff_eq!(l / 2.0, FVec4::splat(0.5));
    }

    #[test]
    fn matrix4() {
        let l = Matrix4::diagonal(1.0, 2.0, 3.0, 4.0);
        let r = Matrix4::identity();

        assert_abs_diff_eq!(l * r, l);
        assert_abs_diff_eq!(l + r, Matrix4::diagonal(2.0, 3.0, 4.0, 5.0));
        assert_abs_diff_eq!(l - r, Matrix4::diagonal(0.0, 1.0, 2.0, 3.0));
    }
}
