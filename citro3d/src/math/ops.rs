use std::borrow::Borrow;
use std::mem::MaybeUninit;
use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

use super::{FVec, FVec3, FVec4, Matrix, Matrix3, Matrix4};

impl Add for FVec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Add(self.0, rhs.0) })
    }
}

impl Sub for FVec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Subtract(self.0, rhs.0) })
    }
}

impl Neg for FVec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Negate(self.0) })
    }
}

impl Mul<f32> for FVec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Scale(self.0, rhs) })
    }
}

impl Add for FVec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Add(self.0, rhs.0) })
    }
}

impl Sub for FVec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Subtract(self.0, rhs.0) })
    }
}

impl Neg for FVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Negate(self.0) })
    }
}

impl Mul<f32> for FVec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Scale(self.0, rhs) })
    }
}

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
        unsafe { self.0.c == other.0.c }
    }
}

impl<const N: usize> Eq for FVec<N> {}

impl<Rhs: Borrow<Self>, const M: usize, const N: usize> Add<Rhs> for &Matrix<M, N> {
    type Output = <Self as Deref>::Target;

    fn add(self, rhs: Rhs) -> Self::Output {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Add(out.as_mut_ptr(), self.as_raw(), rhs.borrow().as_raw());
            Matrix::new(out.assume_init())
        }
    }
}

impl<Rhs: Borrow<Self>, const M: usize, const N: usize> Sub<Rhs> for &Matrix<M, N> {
    type Output = <Self as Deref>::Target;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Subtract(out.as_mut_ptr(), self.as_raw(), rhs.borrow().as_raw());
            Matrix::new(out.assume_init())
        }
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<&Matrix<N, P>> for &Matrix<M, N> {
    type Output = Matrix<M, P>;

    fn mul(self, rhs: &Matrix<N, P>) -> Self::Output {
        let mut out = MaybeUninit::uninit();
        unsafe {
            citro3d_sys::Mtx_Multiply(out.as_mut_ptr(), self.as_raw(), rhs.as_raw());
            Matrix::new(out.assume_init())
        }
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for &Matrix<M, N> {
    type Output = Matrix<M, P>;

    fn mul(self, rhs: Matrix<N, P>) -> Self::Output {
        self * &rhs
    }
}

impl Mul<FVec3> for &Matrix3 {
    type Output = FVec3;

    fn mul(self, rhs: FVec3) -> Self::Output {
        FVec(unsafe { citro3d_sys::Mtx_MultiplyFVec3(self.as_raw(), rhs.0) })
    }
}

impl Mul<FVec4> for &Matrix4 {
    type Output = FVec4;

    fn mul(self, rhs: FVec4) -> Self::Output {
        FVec(unsafe { citro3d_sys::Mtx_MultiplyFVec4(self.as_raw(), rhs.0) })
    }
}

impl Mul<FVec3> for &Matrix<4, 3> {
    type Output = FVec4;

    fn mul(self, rhs: FVec3) -> Self::Output {
        FVec(unsafe { citro3d_sys::Mtx_MultiplyFVecH(self.as_raw(), rhs.0) })
    }
}

impl<Rhs: Borrow<Self>, const M: usize, const N: usize> PartialEq<Rhs> for Matrix<M, N> {
    fn eq(&self, other: &Rhs) -> bool {
        self.as_rows() == other.borrow().as_rows()
    }
}

impl<const M: usize, const N: usize> Eq for Matrix<M, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3() {
        let l = FVec3::splat(1.0);
        let r = FVec3::splat(2.0);

        assert_eq!(l + r, FVec3::splat(3.0));
        assert_eq!(l - r, FVec3::splat(-1.0));
        assert_eq!(-l, FVec3::splat(-1.0));
        assert_eq!(l * 1.5, FVec3::splat(1.5));
        assert_eq!(l / 2.0, FVec3::splat(0.5));
    }

    #[test]
    fn vec4() {
        let l = FVec4::splat(1.0);
        let r = FVec4::splat(2.0);

        assert_eq!(l + r, FVec4::splat(3.0));
        assert_eq!(l - r, FVec4::splat(-1.0));
        assert_eq!(-l, FVec4::splat(-1.0));
        assert_eq!(l * 1.5, FVec4::splat(1.5));
        assert_eq!(l / 2.0, FVec4::splat(0.5));
    }

    #[test]
    fn matrix3() {
        let l = Matrix3::diagonal(1.0, 2.0, 3.0);
        let r = Matrix3::identity();
        let (l, r) = (&l, &r);

        assert_eq!(l * r, l);
        assert_eq!(l + r, Matrix3::diagonal(2.0, 3.0, 4.0));
        assert_eq!(l - r, Matrix3::diagonal(0.0, 1.0, 2.0));
    }

    #[test]
    fn matrix4() {
        let l = Matrix4::diagonal(1.0, 2.0, 3.0, 4.0);
        let r = Matrix4::identity();
        let (l, r) = (&l, &r);

        assert_eq!(l * r, l);
        assert_eq!(l + r, Matrix4::diagonal(2.0, 3.0, 4.0, 5.0));
        assert_eq!(l - r, Matrix4::diagonal(0.0, 1.0, 2.0, 3.0));
    }
}
