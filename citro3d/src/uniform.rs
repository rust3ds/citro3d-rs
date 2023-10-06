use crate::{shader, Instance, Matrix};

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
    use crate::Matrix;

    pub trait Sealed {}
    impl Sealed for Matrix {}
}

pub trait Uniform: private::Sealed {
    fn bind(&self, instance: &mut Instance, shader_type: shader::Type, index: Index);
}

impl Uniform for Matrix {
    fn bind(&self, _instance: &mut Instance, type_: shader::Type, index: Index) {
        unsafe { citro3d_sys::C3D_FVUnifMtx4x4(type_.into(), index.into(), self.as_raw()) }
    }
}
