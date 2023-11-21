//! Common definitions for binding uniforms to shaders. This is primarily
//! done by implementing the [`Uniform`] trait for a given type.

use crate::math::Matrix;
use crate::{shader, Instance};

/// The index of a uniform within a [`shader::Program`].
#[derive(Copy, Clone, Debug)]
pub struct Index(i8);

impl From<i8> for Index {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<Index> for i32 {
    fn from(value: Index) -> Self {
        value.0.into()
    }
}

mod private {
    use crate::math::Matrix;

    pub trait Sealed {}

    impl<const M: usize, const N: usize> Sealed for &Matrix<M, N> {}
}

/// A shader uniform. This trait is implemented for types that can be bound to
/// shaders to be used as a uniform input to the shader.
pub trait Uniform: private::Sealed {
    /// Bind the uniform to the given shader index for the given shader type.
    /// An [`Instance`] is required to prevent concurrent binding of different
    /// uniforms to the same index.
    fn bind(self, instance: &mut Instance, shader_type: shader::Type, index: Index);
}

impl<const M: usize> Uniform for &Matrix<M, 4> {
    #[doc(alias = "C34_FVUnifMtxNx4")]
    #[doc(alias = "C34_FVUnifMtx4x4")]
    #[doc(alias = "C34_FVUnifMtx3x4")]
    #[doc(alias = "C34_FVUnifMtx2x4")]
    fn bind(self, _instance: &mut Instance, type_: shader::Type, index: Index) {
        unsafe {
            citro3d_sys::C3D_FVUnifMtxNx4(
                type_.into(),
                index.into(),
                self.as_raw(),
                // UNWRAP: it should be impossible for end users to construct
                // a matrix with M > i32::MAX
                M.try_into().unwrap(),
            );
        }
    }
}
