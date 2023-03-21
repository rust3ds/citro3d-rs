use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::attrib;

#[derive(Debug)]
pub struct Info(pub(crate) citro3d_sys::C3D_BufInfo);

// TODO: is this a good name? It's more like a "handle" to the VBO data, or a slice.
#[derive(Debug, Clone, Copy)]
pub struct Index<'buf> {
    index: libc::c_int,
    size: libc::c_int,
    _vbo_data: PhantomData<&'buf ()>,
    buf_info: &'buf Info,
}

impl Index<'_> {
    pub fn as_raw(&self) -> libc::c_int {
        self.index
    }

    pub fn size(&self) -> libc::c_int {
        self.size
    }

    pub fn info(&self) -> &Info {
        self.buf_info
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
    pub fn new() -> Self {
        let mut info = MaybeUninit::zeroed();
        let info = unsafe {
            citro3d_sys::BufInfo_Init(info.as_mut_ptr());
            info.assume_init()
        };
        Self(info)
    }

    pub(crate) fn copy_from(raw: *const citro3d_sys::C3D_BufInfo) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            // This is less efficient than returning a pointer or something, but it's
            // safer since we don't know the lifetime of the pointee
            Some(Self(unsafe { *raw }))
        }
    }

    pub fn add<'this, 'vbo, 'idx, T>(
        &'this mut self,
        vbo_data: &'vbo [T],
        attrib_info: &attrib::Info,
    ) -> crate::Result<Index<'idx>>
    where
        'this: 'idx,
        'vbo: 'idx,
    {
        let stride = std::mem::size_of::<T>().try_into()?;
        let attrib_count = attrib_info.count();
        let permutation = attrib_info.permutation();

        // SAFETY: the lifetime of the VBO data is encapsulated in the return value's
        // 'vbo lifetime, and the pointer to &mut self.0 is used to access values
        // in the BufInfo, not copied to be used later.
        let res = unsafe {
            citro3d_sys::BufInfo_Add(
                &mut self.0,
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
                _vbo_data: PhantomData,
                buf_info: self,
            })
        }
    }
}
