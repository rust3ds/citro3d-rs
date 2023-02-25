use std::ops::{Deref, DerefMut};
use std::sync::{LazyLock, RwLock};

static ATTR_INFO: LazyLock<RwLock<AttrInfo>> = LazyLock::new(|| {
    let raw = unsafe {
        // TODO: should we check is_null() here?
        let info = citro3d_sys::C3D_GetAttrInfo();
        citro3d_sys::AttrInfo_Init(info);
        info
    };

    RwLock::new(AttrInfo { raw })
});

pub struct AttrInfo {
    raw: *mut citro3d_sys::C3D_AttrInfo,
}

pub struct Register(libc::c_int);

impl Register {
    pub fn new(n: u16) -> crate::Result<Self> {
        // TODO proper validation for attributes? Or maybe just a next() function
        // that gets atomically increasing indices or something? Or look at
        // <https://3dbrew.org/wiki/GPU/Internal_Registers> and define some consts
        // or lookup functions
        Ok(Self(n as _))
    }
}

#[must_use]
pub struct Index(libc::c_int);

#[repr(u32)]
pub enum Format {
    Byte = ctru_sys::GPU_BYTE,
    UnsignedByte = ctru_sys::GPU_UNSIGNED_BYTE,
    Float = ctru_sys::GPU_FLOAT,
    Short = ctru_sys::GPU_SHORT,
}

// SAFETY: the RWLock ensures unique access when mutating the global struct, and
// we trust citro3d to Do The Right Thing™ and not mutate it otherwise.
unsafe impl Sync for AttrInfo {}
unsafe impl Send for AttrInfo {}

impl AttrInfo {
    /// Get a reference to the global attribute info.
    pub fn get() -> crate::Result<impl Deref<Target = Self>> {
        Ok(ATTR_INFO.try_read()?)
    }

    /// Get a mutable reference to the global attribute info.
    pub fn get_mut() -> crate::Result<impl DerefMut<Target = Self>> {
        Ok(ATTR_INFO.try_write()?)
    }

    /// Add an attribute loader to the attribute info. By default, the resulting
    /// attribute index will be appended to the permutation
    pub fn add_loader(
        &mut self,
        register: Register,
        format: Format,
        count: usize,
    ) -> crate::Result<Index> {
        let count = count.try_into()?;

        let idx =
            unsafe { citro3d_sys::AttrInfo_AddLoader(self.raw, register.0, format as u32, count) };

        Ok(Index(idx))
    }

    pub fn set_permutation(&mut self, indices: &[Index]) -> crate::Result<()> {
        if indices.len() > 16 {
            return Err(crate::Error::TooManyAttributes);
        }

        let mut bytes: Vec<u8> = indices
            .windows(2)
            .map(|window| {
                let [lo, hi] = match *window {
                    [Index(lo), Index(hi)] => [lo, hi],
                    [Index(lo)] => [lo, 0], // high nibble is just padding
                    _ => unreachable!(),    // window size of 2 == always 1 or 2 elements
                };
                // each value is a nibble, combine them into a byte
                lo as u8 | (hi as u8) << 4
            })
            .collect();

        // pad the remainder with zeros
        bytes.extend(std::iter::repeat(0).take(8 - bytes.len()));

        let permutation = bytemuck::cast(<[u8; 8]>::try_from(bytes).unwrap());

        unsafe {
            (*self.raw).permutation = permutation;
            (*self.raw).attrCount = indices.len() as _;
        }
        Ok(())
    }

    /// Get the current permutation of input register to vertex attributes mapping.
    /// See [GPU/Internal Registers] for an explanation of how the bits are laid out
    /// in the resulting value.
    ///
    /// [GPU/Internal Registers]: https://3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_SH_ATTRIBUTES_PERMUTATION_LOW
    pub fn permutation(&self) -> u64 {
        unsafe { (*self.raw).permutation }
    }

    /// Get the number of attributes in the current permutation.
    pub fn count(&self) -> libc::c_int {
        unsafe { (*self.raw).attrCount }
    }
}
