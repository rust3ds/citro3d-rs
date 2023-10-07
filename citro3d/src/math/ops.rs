use std::borrow::Borrow;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{FVec, FVec3, FVec4};

impl<Rhs: Borrow<Self>> Add<Rhs> for FVec4 {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Add(self.0, rhs.borrow().0) })
    }
}

impl<Rhs: Borrow<Self>> Sub<Rhs> for FVec4 {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec4_Add(self.0, rhs.borrow().0) })
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

impl<Rhs: Borrow<Self>> Add<Rhs> for FVec3 {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Add(self.0, rhs.borrow().0) })
    }
}

impl<Rhs: Borrow<Self>> Sub<Rhs> for FVec3 {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Self(unsafe { citro3d_sys::FVec3_Add(self.0, rhs.borrow().0) })
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

#[cfg(test)]
mod tests {
    #![allow(clippy::op_ref)]

    use super::*;

    #[test]
    fn vec3_ops() {
        let l = FVec3::splat(1.0);
        let r = FVec3::splat(2.0);

        assert_eq!(l + r, FVec3::splat(3.0));
        assert_eq!(l + &r, FVec3::splat(3.0));
        assert_eq!(l - r, FVec3::splat(-1.0));
        assert_eq!(l - &r, FVec3::splat(-1.0));
        assert_eq!(-l, FVec3::splat(-1.0));
        assert_eq!(l * 1.5, FVec3::splat(1.5));
        assert_eq!(l / 2.0, FVec3::splat(0.5));
    }

    #[test]
    fn vec4_ops() {
        let l = FVec4::splat(1.0);
        let r = FVec4::splat(2.0);

        assert_eq!(l + r, FVec4::splat(3.0));
        assert_eq!(l + &r, FVec4::splat(3.0));
        assert_eq!(l - r, FVec4::splat(-1.0));
        assert_eq!(l - &r, FVec4::splat(-1.0));
        assert_eq!(-l, FVec4::splat(-1.0));
        assert_eq!(l * 1.5, FVec4::splat(1.5));
        assert_eq!(l / 2.0, FVec4::splat(0.5));
    }
}
