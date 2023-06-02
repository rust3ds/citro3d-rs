use std::mem::MaybeUninit;

/// Vertex attribute info. This struct is used to describe how vertex buffers are
/// used (i.e. the shape of the vertex data).
#[derive(Debug)]
pub struct Info(pub(crate) citro3d_sys::C3D_AttrInfo);

/// A shader input register, usually corresponding to a single vertex attribute
/// (e.g. position or color). These are called `v0`, `v1`, ... `v15` in the
/// [picasso](https://github.com/devkitPro/picasso/blob/master/Manual.md)
/// shader language.
#[derive(Debug, Clone, Copy)]
pub struct Register(libc::c_int);

impl Register {
    /// Get a register corresponding to the given index.
    ///
    /// # Errors
    ///
    /// Returns an error for `n >= 16`.
    pub fn new(n: u16) -> crate::Result<Self> {
        if n < 16 {
            Ok(Self(n.into()))
        } else {
            Err(crate::Error::TooManyAttributes)
        }
    }
}

/// An attribute index. This is the attribute's actual index in the input buffer,
/// and may correspond to any [`Register`] (or multiple) as input in the shader
/// program.
#[derive(Debug, Clone, Copy)]
pub struct Index(u8);

/// The data format of an attribute.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Format {
    /// A signed byte, i.e. [`i8`].
    Byte = ctru_sys::GPU_BYTE,
    /// An unsigned byte, i.e. [`u8`].
    UnsignedByte = ctru_sys::GPU_UNSIGNED_BYTE,
    /// A float, i.e. [`f32`].
    Float = ctru_sys::GPU_FLOAT,
    /// A short integer, i.e. [`i16`].
    Short = ctru_sys::GPU_SHORT,
}

// SAFETY: the RWLock ensures unique access when mutating the global struct, and
// we trust citro3d to Do The Right Thing™ and not mutate it otherwise.
unsafe impl Sync for Info {}
unsafe impl Send for Info {}

impl Default for Info {
    fn default() -> Self {
        let mut raw = MaybeUninit::zeroed();
        let raw = unsafe {
            citro3d_sys::AttrInfo_Init(raw.as_mut_ptr());
            raw.assume_init()
        };
        Self(raw)
    }
}

impl Info {
    /// Construct a new attribute info structure with no attributes.
    pub fn new() -> Self {
        Self::default()
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

    /// Add an attribute loader to the attribute info. The resulting attribute index
    /// indicates the registration order of the attributes.
    ///
    /// ## Parameters
    ///
    /// * `register`: the shader program input register for this attribute.
    /// * `format`: the data format of this attribute.
    /// * `count`: the number of elements in each attribute (up to 4, corresponding
    ///   to `xyzw` / `rgba` / `stpq`).
    ///
    /// ## Errors
    ///
    /// * If `count > 4`
    /// * If this attribute info already has the maximum number of attributes.
    pub fn add_loader(
        &mut self,
        register: Register,
        format: Format,
        count: u8,
    ) -> crate::Result<Index> {
        if count > 4 {
            return Err(crate::Error::InvalidSize);
        }

        // SAFETY: the &mut self.0 reference is only used to access fields in
        // the attribute info, not stored somewhere for later use
        let ret = unsafe {
            citro3d_sys::AttrInfo_AddLoader(&mut self.0, register.0, format as u32, count.into())
        };

        let Ok(idx) = ret.try_into() else {
            return Err(crate::Error::TooManyAttributes)
        };

        Ok(Index(idx))
    }

    pub(crate) fn permutation(&self) -> u64 {
        self.0.permutation
    }

    /// Get the number of registered attributes.
    pub fn attr_count(&self) -> libc::c_int {
        self.0.attrCount
    }
}
