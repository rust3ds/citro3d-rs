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

impl BufInfo {
    /// Get a reference to the global buffer info.
    pub fn get() -> crate::Result<impl Deref<Target = Self>> {
        Ok(BUF_INFO.try_read()?)
    }

    /// Get a mutable reference to the global buffer info.
    pub fn get_mut() -> crate::Result<impl DerefMut<Target = Self>> {
        Ok(BUF_INFO.try_write()?)
    }

    pub fn add<T>(&mut self, vbo_data: &[T], attrib_info: &attrib::AttrInfo) -> crate::Result<()> {
        let stride = std::mem::size_of::<T>().try_into()?;
        let attrib_count = attrib_info.count();
        let permutation = attrib_info.permutation();

        unsafe {
            citro3d_sys::BufInfo_Add(
                self.raw,
                // TODO: figure out how the hell to encode the lifetime of this
                // data so that we don't try to use it after it's destroyed...
                vbo_data.as_ptr().cast(),
                stride,
                attrib_count,
                permutation,
            );
        }

        Ok(())
    }
}
