use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct Info(pub(crate) citro3d_sys::C3D_AttrInfo);

#[derive(Debug)]
pub struct Register(libc::c_int);

impl Register {
    pub fn new(n: u16) -> crate::Result<Self> {
        // TODO proper validation for attributes? Or maybe just a next() function
        // that gets atomically increasing indices or something? Or look at
        // <https://3dbrew.org/wiki/GPU/Internal_Registers> and define some consts
        // or lookup functions
        Ok(Self(n.into()))
    }
}

#[must_use]
pub struct Index(u8);

#[repr(u32)]
pub enum Format {
    Byte = ctru_sys::GPU_BYTE,
    UnsignedByte = ctru_sys::GPU_UNSIGNED_BYTE,
    Float = ctru_sys::GPU_FLOAT,
    Short = ctru_sys::GPU_SHORT,
}

// SAFETY: the RWLock ensures unique access when mutating the global struct, and
// we trust citro3d to Do The Right Thing™ and not mutate it otherwise.
unsafe impl Sync for Info {}
unsafe impl Send for Info {}

impl Info {
    pub fn new() -> Self {
        let mut raw = MaybeUninit::zeroed();
        let raw = unsafe {
            citro3d_sys::AttrInfo_Init(raw.as_mut_ptr());
            raw.assume_init()
        };
        Self(raw)
    }

    pub(crate) fn copy_from(raw: *const citro3d_sys::C3D_AttrInfo) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            // This is less efficient than returning a pointer or something, but it's
            // safer since we don't know the lifetime of the pointee
            Some(Self(unsafe { *raw }))
        }
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

        // SAFETY: the &mut self.0 reference is only used to access fields in
        // the attribute info, not stored somewhere for later use
        let ret = unsafe {
            citro3d_sys::AttrInfo_AddLoader(&mut self.0, register.0, format as u32, count)
        };

        let Ok(idx) = ret.try_into() else {
            return Err(crate::Error::FailedToInitialize)
        };

        Ok(Index(idx))
    }

    pub fn set_permutation(&mut self, indices: &[Index]) -> crate::Result<()> {
        if indices.len() > 16 {
            return Err(crate::Error::TooManyAttributes);
        }
        let attr_count: libc::c_int = indices.len().try_into().unwrap();

        let mut bytes: Vec<u8> = indices
            .windows(2)
            .map(|window| {
                let [lo, hi] = match *window {
                    [Index(lo), Index(hi)] => [lo, hi],
                    [Index(lo)] => [lo, 0], // high nibble is just padding
                    _ => unreachable!(),    // window size of 2 == always 1 or 2 elements
                };
                // each value is a nibble, combine them into a byte
                lo | (hi << 4)
            })
            .collect();

        // pad the remainder with zeros
        bytes.extend(std::iter::repeat(0).take(8 - bytes.len()));

        let permutation = bytemuck::cast(<[u8; 8]>::try_from(bytes).unwrap());

        self.0.permutation = permutation;
        self.0.attrCount = attr_count;

        Ok(())
    }

    /// Get the current permutation of input register to vertex attributes mapping.
    /// See [GPU/Internal Registers] for an explanation of how the bits are laid out
    /// in the resulting value.
    ///
    /// [GPU/Internal Registers]: https://3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_SH_ATTRIBUTES_PERMUTATION_LOW
    pub fn permutation(&self) -> u64 {
        self.0.permutation
    }

    /// Get the number of attributes in the current permutation.
    pub fn count(&self) -> libc::c_int {
        self.0.attrCount
    }
}
