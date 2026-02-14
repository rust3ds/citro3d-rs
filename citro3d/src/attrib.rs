//! Configure vertex attributes.
//!
//! This module has types and helpers for describing the shape/structure of vertex
//! data to be sent to the GPU.
//!
//! See the [`buffer`](crate::buffer) module to use the vertex data itself.

use std::mem::MaybeUninit;

/// A shader input register, usually corresponding to a single vertex attribute
/// (e.g. position or color). These are called `v0`, `v1`, ... `v15` in the
/// [picasso](https://github.com/devkitPro/picasso/blob/master/Manual.md)
/// shader language.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Register {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    V10 = 10,
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
}

/// The permutation of a buffer containing vertex attribute data.
/// The Permutation maps the layout of an input buffer's fields to the
/// input registers used in the picasso shader.
#[derive(Debug, Copy, Clone)]
pub struct Permutation {
    pub(crate) permutation: u64,
    pub(crate) attrib_count: u8,
}

impl Permutation {
    /// Construct the permutation for a buffer whos fields (in order) correspond to the
    /// provided list of input registers (as used in the picasso shader).
    ///
    /// # Example
    /// ## Picasso
    /// ```
    /// ; Inputs (defined as aliases for convenience)
    /// .alias inpos         v0 ; fvec3
    /// .alias innorm        v1 ; fvec3
    /// .alias intex         v2 ; fvec2
    /// ```
    ///
    /// ## Rust
    ///
    /// ```
    /// struct Vertex {
    ///     pos: [f32; 3],
    ///     tex: [f32; 2],
    /// }
    ///
    /// impl Vertex {
    ///     pub fn permutation() -> Permutation {
    ///         Permutation::from_layout(&[Register::V0, Register::V2]).unwrap()
    ///     }
    /// }
    ///
    /// struct Normal {
    ///     norm: [f32; 3],
    /// }
    ///
    /// impl Normal {
    ///     pub fn permutation() -> Permutation {
    ///         Permutation::from_layout(&[Register::V1]).unwrap()
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If more than 16 attribute registers are provided (i.e. `layout.len() > 16`)
    pub fn from_layout(layout: &[Register]) -> Result<Self, crate::Error> {
        if layout.len() > 16 {
            return Err(crate::Error::IndexOutOfBounds {
                idx: (layout.len() - 1) as i32,
                len: 16,
            });
        }

        let mut perm: u64 = 0;

        for (i, l) in layout.iter().enumerate() {
            perm |= (*l as u64) << (i * 4);
        }

        Ok(Self {
            permutation: perm,
            attrib_count: layout.len() as u8,
        })
    }
}

/// Vertex attribute info. This struct describes how vertex buffers are
/// layed out and used (i.e. the shape of the vertex data).
#[derive(Debug)]
#[doc(alias = "C3D_AttrInfo")]
pub struct Info(pub(crate) citro3d_sys::C3D_AttrInfo);

/// An attribute index. This is the attribute's actual index in the input buffer,
/// and may correspond to any [`Register`] (or multiple) as input in the shader
/// program.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Index(u8);

/// The data format of an attribute.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[doc(alias = "GPU_FORMATS")]
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

impl From<Format> for u8 {
    fn from(value: Format) -> Self {
        value as u8
    }
}

// SAFETY: the RWLock ensures unique access when mutating the global struct, and
// we trust citro3d to Do The Right Thing™ and not mutate it otherwise.
unsafe impl Sync for Info {}
unsafe impl Send for Info {}

impl Default for Info {
    #[doc(alias = "AttrInfo_Init")]
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
    /// # Parameters
    ///
    /// * `register`: the shader program input register for this attribute.
    /// * `format`: the data format of this attribute.
    /// * `count`: the number of elements in each attribute (up to 4, corresponding
    ///   to `xyzw` / `rgba` / `stpq`).
    ///
    /// # Errors
    ///
    /// * If `count > 4`
    /// * If this attribute info already has the maximum number of attributes.
    #[doc(alias = "AttrInfo_AddLoader")]
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
            citro3d_sys::AttrInfo_AddLoader(&mut self.0, register as _, format.into(), count.into())
        };

        let Ok(idx) = ret.try_into() else {
            return Err(crate::Error::TooManyAttributes);
        };

        Ok(Index(idx))
    }

    /// Get the [`Permutation`] for a buffer with elements containing all the fields
    /// added to this `Info`. If the buffer elements do not contain all the fields
    /// or contain them in a different order than they were added to this `Info`,
    /// construct the [`Permutation`] yourself using [`Permutation::from_layout`].
    pub fn permutation(&self) -> Permutation {
        Permutation {
            permutation: self.0.permutation,
            attrib_count: self.attr_count() as u8,
        }
    }

    /// Get the number of registered attributes.
    pub fn attr_count(&self) -> libc::c_int {
        self.0.attrCount
    }
}
