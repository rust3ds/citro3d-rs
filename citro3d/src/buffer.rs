use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::{LazyLock, RwLock};

use crate::attrib;

static BUF_INFO: LazyLock<RwLock<Info>> = LazyLock::new(|| {
    let raw = unsafe {
        let info = citro3d_sys::C3D_GetBufInfo();
        citro3d_sys::BufInfo_Init(info);
        info
    };

    RwLock::new(Info { raw })
});

/// Vertex attribute info. This struct can be used to
pub struct Info {
    raw: *mut citro3d_sys::C3D_BufInfo,
}

// SAFETY: the RWLock ensures unique access when mutating the global struct, and
// we trust citro3d to Do The Right Thing™ and not mutate it otherwise.
unsafe impl Sync for Info {}
unsafe impl Send for Info {}

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

impl Info {
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
        attrib_info: &attrib::Info,
    ) -> crate::Result<Index<'vbo>> {
        let stride = std::mem::size_of::<T>().try_into()?;
        let attrib_count = attrib_info.count();
        let permutation = attrib_info.permutation();

        let res = unsafe {
            citro3d_sys::BufInfo_Add(
                self.raw,
                vbo_data.as_ptr().cast(),
                stride,
                attrib_count,
                permutation,
            )
        };

        if res < 0 {
            // TODO: should we convert to a more specific error if this fails?
            // It looks like the common cases are
            // - too many buffers already added (max 12)
            // - physical memory address in the wrong place (this can be seen by
            //   using default allocator instead of LinearAllocator)
            // <https://github.com/devkitPro/citro3d/blob/master/source/buffers.c#L13-L17>
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
