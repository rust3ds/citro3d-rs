//! Functionality for parsing and using PICA200 shaders on the 3DS. This module
//! does not compile shaders, but enables using pre-compiled shaders at runtime.
//!
//! For more details about the PICA200 compiler / shader language, see
//! documentation for <https://github.com/devkitPro/picasso>.

use std::error::Error;
use std::mem::MaybeUninit;

pub mod macros;

/// A PICA200 shader program. It may have one or both of:
///
/// * A vertex [shader instance](Instance)
/// * A geometry [shader instance](Instance)
///
/// The PICA200 does not support user-programmable fragment shaders.
pub struct Program {
    program: citro3d_sys::shaderProgram_s,
}

impl Program {
    /// Create a new shader program from a vertex shader.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * the shader program cannot be initialized
    /// * the input shader is not a vertex shader or is otherwise invalid
    pub fn new(vertex_shader: Entrypoint) -> Result<Self, ctru::Error> {
        let mut program = unsafe {
            let mut program = MaybeUninit::uninit();
            let result = citro3d_sys::shaderProgramInit(program.as_mut_ptr());
            if result != 0 {
                return Err(ctru::Error::from(result));
            }
            program.assume_init()
        };

        let ret = unsafe { citro3d_sys::shaderProgramSetVsh(&mut program, vertex_shader.as_raw()) };

        if ret == 0 {
            Ok(Self { program })
        } else {
            Err(ctru::Error::from(ret))
        }
    }

    /// Set the geometry shader for a given program.
    ///
    /// # Errors
    ///
    /// Returns an error if the input shader is not a geometry shader or is
    /// otherwise invalid.
    pub fn set_geometry_shader(
        &mut self,
        geometry_shader: Entrypoint,
        stride: u8,
    ) -> Result<(), ctru::Error> {
        let ret = unsafe {
            citro3d_sys::shaderProgramSetGsh(&mut self.program, geometry_shader.as_raw(), stride)
        };

        if ret == 0 {
            Ok(())
        } else {
            Err(ctru::Error::from(ret))
        }
    }

    // TODO: pub(crate)
    pub fn as_raw(&mut self) -> *mut citro3d_sys::shaderProgram_s {
        &mut self.program
    }
}

impl<'vert, 'geom> Drop for Program {
    fn drop(&mut self) {
        unsafe {
            let _ = citro3d_sys::shaderProgramFree(self.as_raw());
        }
    }
}

/// A PICA200 Shader Library (commonly called DVLB). This can be comprised of
/// one or more [`Entrypoint`]s, but most commonly has one vertex shader and an
/// optional geometry shader.
///
/// This is the result of parsing a shader binary (shbin), and the resulting
/// [`Entrypoint`]s can be used as part of a [`Program`].
pub struct Library(*mut citro3d_sys::DVLB_s);

impl Library {
    /// Parse a new shader library from input bytes.
    ///
    /// # Errors
    ///
    /// An error is returned if the input data does not have an alignment of 4
    /// (cannot be safely converted to `&[u32]`).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let aligned: &[u32] = bytemuck::try_cast_slice(bytes)?;
        Ok(Self(unsafe {
            citro3d_sys::DVLB_ParseFile(
                // SAFETY: we're trusting the parse implementation doesn't mutate
                // the contents of the data. From a quick read it looks like that's
                // correct and it should just take a const arg in the API.
                aligned.as_ptr() as *mut _,
                aligned.len().try_into()?,
            )
        }))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { (*self.0).numDVLE as usize }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<Entrypoint> {
        if index < self.len() {
            Some(Entrypoint {
                ptr: unsafe { (*self.0).DVLE.add(index) },
                _library: self,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_raw(&mut self) -> *mut citro3d_sys::DVLB_s {
        self.0
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            citro3d_sys::DVLB_Free(self.as_raw());
        }
    }
}

/// A shader library entrypoint (also called DVLE). This represents either a
/// vertex or a geometry shader.
#[derive(Clone, Copy)]
pub struct Entrypoint<'lib> {
    ptr: *mut citro3d_sys::DVLE_s,
    _library: &'lib Library,
}

impl<'lib> Entrypoint<'lib> {
    fn as_raw(self) -> *mut citro3d_sys::DVLE_s {
        self.ptr
    }
}
