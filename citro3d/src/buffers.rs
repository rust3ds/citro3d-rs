use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{LazyLock, RwLock};

use crate::attrib;

static BUF_INFO: LazyLock<RwLock<BufInfo>> = LazyLock::new(|| {
    let raw = unsafe {
        let info = citro3d_sys::C3D_GetBufInfo();
        citro3d_sys::BufInfo_Init(info);
        info
    };

    RwLock::new(BufInfo { raw })
});

pub struct BufInfo {
    raw: *mut citro3d_sys::C3D_BufInfo,
}

// SAFETY: the RWLock ensures unique access when mutating the global struct, and
// we trust citro3d to Do The Right Thing™ and not mutate it otherwise.
unsafe impl Sync for BufInfo {}
unsafe impl Send for BufInfo {}

// TODO: is this a good name? It's more like a "handle" to the VBO data, or a slice.
#[derive(Debug, Clone, Copy)]
pub struct Index<'vbo> {
    index: libc::c_int,
    size: libc::c_int,
    _data: PhantomData<&'vbo ()>,
}

impl Index<'_> {
    pub fn as_raw(&self) -> libc::c_int {
        self.index
    }

    pub fn size(&self) -> libc::c_int {
        self.size
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Triangles = ctru_sys::GPU_TRIANGLES,
    TriangleStrip = ctru_sys::GPU_TRIANGLE_STRIP,
    TriangleFan = ctru_sys::GPU_TRIANGLE_FAN,
    GeometryPrim = ctru_sys::GPU_GEOMETRY_PRIM,
}

impl BufInfo {
    /// Get a reference to the global buffer info.
    pub fn get() -> crate::Result<impl Deref<Target = Self>> {
        Ok(BUF_INFO.try_read()?)
    }

    /// Get a mutable reference to the global buffer info.
    pub fn get_mut() -> crate::Result<impl DerefMut<Target = Self>> {
        Ok(BUF_INFO.try_write()?)
    }

    pub fn add<'vbo, T>(
        &mut self,
        vbo_data: &'vbo [T],
        attrib_info: &attrib::AttrInfo,
    ) -> crate::Result<Index<'vbo>> {
        let stride = std::mem::size_of::<T>().try_into()?;
        let attrib_count = attrib_info.count();
        let permutation = attrib_info.permutation();

        let res = unsafe {
            citro3d_sys::BufInfo_Add(
                self.raw,
                // TODO: figure out how the hell to encode the lifetime of this
                // data so that we don't try to use it after it's destroyed...
                vbo_data.as_ptr().cast(),
                stride,
                attrib_count,
                permutation,
            )
        };

        if res < 0 {
            Err(crate::Error::System(res))
        } else {
            Ok(Index {
                index: res,
                size: vbo_data.len().try_into()?,
                _data: PhantomData,
            })
        }
    }
}
