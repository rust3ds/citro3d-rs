//! This module provides render target types and options for controlling transfer
//! of data to the GPU, including the format of color and depth data to be rendered.

use std::cell::{OnceCell, RefMut};
use std::marker::PhantomData;
use std::pin::Pin;
use std::rc::Rc;

use citro3d_sys::{
    C3D_DEPTHTYPE, C3D_RenderTarget, C3D_RenderTargetCreate, C3D_RenderTargetDelete,
};
use ctru::services::gfx::Screen;
use ctru::services::gspgpu::FramebufferFormat;
use ctru_sys::{GPU_COLORBUF, GPU_DEPTHBUF};

use crate::{
    Error, Instance, RenderQueue, Result, attrib,
    buffer::{self, Index, Indices},
    light::LightEnv,
    shader,
    texenv::{self, TexEnv},
    uniform::{self, Uniform},
};

pub mod effect;
mod transfer;

bitflags::bitflags! {
    /// Indicate whether color, depth buffer, or both values should be cleared.
    #[doc(alias = "C3D_ClearBits")]
    pub struct ClearFlags: u8 {
        /// Clear the color of the render target.
        const COLOR = citro3d_sys::C3D_CLEAR_COLOR;
        /// Clear the depth buffer value of the render target.
        const DEPTH = citro3d_sys::C3D_CLEAR_DEPTH;
        /// Clear both color and depth buffer values of the render target.
        const ALL = citro3d_sys::C3D_CLEAR_ALL;
    }
}

/// The color format to use when rendering on the GPU.
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[doc(alias = "GPU_COLORBUF")]
pub enum ColorFormat {
    /// 8-bit Red + 8-bit Green + 8-bit Blue + 8-bit Alpha.
    RGBA8 = ctru_sys::GPU_RB_RGBA8,
    /// 8-bit Red + 8-bit Green + 8-bit Blue.
    RGB8 = ctru_sys::GPU_RB_RGB8,
    /// 5-bit Red + 5-bit Green + 5-bit Blue + 1-bit Alpha.
    RGBA5551 = ctru_sys::GPU_RB_RGBA5551,
    /// 5-bit Red + 6-bit Green + 5-bit Blue.
    RGB565 = ctru_sys::GPU_RB_RGB565,
    /// 4-bit Red + 4-bit Green + 4-bit Blue + 4-bit Alpha.
    RGBA4 = ctru_sys::GPU_RB_RGBA4,
}

/// The depth buffer format to use when rendering.
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[doc(alias = "GPU_DEPTHBUF")]
#[doc(alias = "C3D_DEPTHTYPE")]
pub enum DepthFormat {
    /// 16-bit depth.
    Depth16 = ctru_sys::GPU_RB_DEPTH16,
    /// 24-bit depth.
    Depth24 = ctru_sys::GPU_RB_DEPTH24,
    /// 24-bit depth + 8-bit Stencil.
    Depth24Stencil8 = ctru_sys::GPU_RB_DEPTH24_STENCIL8,
}

/// A render target for `citro3d`. Frame data will be written to this target
/// to be rendered on the GPU and displayed on the screen.
#[doc(alias = "C3D_RenderTarget")]
pub struct Target<'screen> {
    raw: *mut citro3d_sys::C3D_RenderTarget,
    // This is unused after construction, but ensures unique access to the
    // screen this target writes to during rendering
    _screen: RefMut<'screen, dyn Screen>,
    _queue: Rc<RenderQueue>,
}

#[non_exhaustive]
#[must_use]
pub struct RenderPass<'pass> {
    texenvs: [OnceCell<TexEnv>; texenv::TEXENV_COUNT],
    _phantom: PhantomData<&'pass mut Instance>,
}

impl<'pass> RenderPass<'pass> {
    pub(crate) fn new(_istance: &'pass mut Instance) -> Self {
        Self {
            texenvs: [
                // thank goodness there's only six of them!
                OnceCell::new(),
                OnceCell::new(),
                OnceCell::new(),
                OnceCell::new(),
                OnceCell::new(),
                OnceCell::new(),
            ],
            _phantom: PhantomData,
        }
    }

    /// Select the given render target for drawing the frame. This must be called
    /// as pare of a render call (i.e. within the call to
    /// [`render_frame_with`](Self::render_frame_with)).
    ///
    /// # Errors
    ///
    /// Fails if the given target cannot be used for drawing, or called outside
    /// the context of a frame render.
    #[doc(alias = "C3D_FrameDrawOn")]
    pub fn select_render_target(&mut self, target: &'pass Target<'_>) -> Result<()> {
        let _ = self;
        if unsafe { citro3d_sys::C3D_FrameDrawOn(target.as_raw()) } {
            Ok(())
        } else {
            Err(Error::InvalidRenderTarget)
        }
    }

    /// Get the buffer info being used, if it exists. Note that the resulting
    /// [`buffer::Info`] is copied from the one currently in use.
    #[doc(alias = "C3D_GetBufInfo")]
    pub fn buffer_info(&self) -> Option<buffer::Info> {
        let raw = unsafe { citro3d_sys::C3D_GetBufInfo() };
        buffer::Info::copy_from(raw)
    }

    /// Set the buffer info to use for any following draw calls.
    #[doc(alias = "C3D_SetBufInfo")]
    pub fn set_buffer_info(&mut self, buffer_info: &buffer::Info) {
        let raw: *const _ = &buffer_info.0;
        // LIFETIME SAFETY: C3D_SetBufInfo actually copies the pointee instead of mutating it.
        unsafe { citro3d_sys::C3D_SetBufInfo(raw.cast_mut()) };
    }

    /// Get the attribute info being used, if it exists. Note that the resulting
    /// [`attrib::Info`] is copied from the one currently in use.
    #[doc(alias = "C3D_GetAttrInfo")]
    pub fn attr_info(&self) -> Option<attrib::Info> {
        let raw = unsafe { citro3d_sys::C3D_GetAttrInfo() };
        attrib::Info::copy_from(raw)
    }

    /// Set the attribute info to use for any following draw calls.
    #[doc(alias = "C3D_SetAttrInfo")]
    pub fn set_attr_info(&mut self, attr_info: &attrib::Info) {
        let raw: *const _ = &attr_info.0;
        // LIFETIME SAFETY: C3D_SetAttrInfo actually copies the pointee instead of mutating it.
        unsafe { citro3d_sys::C3D_SetAttrInfo(raw.cast_mut()) };
    }

    /// Render primitives from the current vertex array buffer.
    #[doc(alias = "C3D_DrawArrays")]
    pub fn draw_arrays(&mut self, primitive: buffer::Primitive, vbo_data: buffer::Slice<'pass>) {
        self.set_buffer_info(vbo_data.info());

        // TODO: should we also require the attrib info directly here?
        unsafe {
            citro3d_sys::C3D_DrawArrays(
                primitive as ctru_sys::GPU_Primitive_t,
                vbo_data.index(),
                vbo_data.len(),
            );
        }
    }

    /// Indexed drawing. Draws the vertices in `buf` indexed by `indices`.
    #[doc(alias = "C3D_DrawElements")]
    pub fn draw_elements<I: Index>(
        &mut self,
        primitive: buffer::Primitive,
        vbo_data: buffer::Slice<'pass>,
        indices: &Indices<'pass, I>,
    ) {
        self.set_buffer_info(vbo_data.info());

        let indices = &indices.buffer;
        let elements = indices.as_ptr().cast();

        unsafe {
            citro3d_sys::C3D_DrawElements(
                primitive as ctru_sys::GPU_Primitive_t,
                indices.len().try_into().unwrap(),
                // flag bit for short or byte
                I::TYPE,
                elements,
            );
        }
    }

    /// Use the given [`shader::Program`] for subsequent draw calls.
    pub fn bind_program(&mut self, program: &'pass shader::Program) {
        // SAFETY: AFAICT C3D_BindProgram just copies pointers from the given program,
        // instead of mutating the pointee in any way that would cause UB
        unsafe {
            citro3d_sys::C3D_BindProgram(program.as_raw().cast_mut());
        }
    }

    /// Binds a [`LightEnv`] for the following draw calls.
    pub fn bind_light_env(&mut self, env: Option<&'pass mut Pin<Box<LightEnv>>>) {
        unsafe {
            citro3d_sys::C3D_LightEnvBind(
                env.map_or(std::ptr::null_mut(), |env| env.as_mut().as_raw_mut()),
            );
        }
    }

    /// Bind a uniform to the given `index` in the vertex shader for the next draw call.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::uniform;
    /// # use citro3d::math::Matrix4;
    /// #
    /// # let mut instance = citro3d::Instance::new().unwrap();
    /// let idx = uniform::Index::from(0);
    /// let mtx = Matrix4::identity();
    /// instance.bind_vertex_uniform(idx, &mtx);
    /// ```
    pub fn bind_vertex_uniform(&mut self, index: uniform::Index, uniform: impl Into<Uniform>) {
        // LIFETIME SAFETY: Uniform data is copied into global buffers.
        uniform.into().bind(self, shader::Type::Vertex, index);
    }

    /// Bind a uniform to the given `index` in the geometry shader for the next draw call.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use citro3d::uniform;
    /// # use citro3d::math::Matrix4;
    /// #
    /// # let mut instance = citro3d::Instance::new().unwrap();
    /// let idx = uniform::Index::from(0);
    /// let mtx = Matrix4::identity();
    /// instance.bind_geometry_uniform(idx, &mtx);
    /// ```
    pub fn bind_geometry_uniform(&mut self, index: uniform::Index, uniform: impl Into<Uniform>) {
        // LIFETIME SAFETY: Uniform data is copied into global buffers.
        uniform.into().bind(self, shader::Type::Geometry, index);
    }

    /// Retrieve the [`TexEnv`] for the given stage, initializing it first if necessary.
    ///
    /// # Example
    ///
    /// ```
    /// # use citro3d::texenv;
    /// # let _runner = test_runner::GdbRunner::default();
    /// # let mut instance = citro3d::Instance::new().unwrap();
    /// let stage0 = texenv::Stage::new(0).unwrap();
    /// let texenv0 = instance.texenv(stage0);
    /// ```
    #[doc(alias = "C3D_GetTexEnv")]
    #[doc(alias = "C3D_TexEnvInit")]
    pub fn texenv(&mut self, stage: texenv::Stage) -> &mut texenv::TexEnv {
        let texenv = &mut self.texenvs[stage.0];
        texenv.get_or_init(|| TexEnv::new(stage));
        // We have to do this weird unwrap to get a mutable reference,
        // since there is no `get_mut_or_init` or equivalent
        texenv.get_mut().unwrap()
    }
}

impl<'screen> Target<'screen> {
    /// Create a new render target with the given parameters. This takes a
    /// [`RenderQueue`] parameter to make sure this  [`Target`] doesn't outlive
    /// the render queue.
    pub(crate) fn new(
        width: usize,
        height: usize,
        screen: RefMut<'screen, dyn Screen>,
        depth_format: Option<DepthFormat>,
        queue: Rc<RenderQueue>,
    ) -> Result<Self> {
        let color_format: ColorFormat = screen.framebuffer_format().into();

        let raw = unsafe {
            C3D_RenderTargetCreate(
                width.try_into()?,
                height.try_into()?,
                color_format as GPU_COLORBUF,
                depth_format.map_or(C3D_DEPTHTYPE { __i: -1 }, DepthFormat::as_raw),
            )
        };

        if raw.is_null() {
            return Err(Error::FailedToInitialize);
        }

        // Set the render target to actually output to the given screen
        let flags = transfer::Flags::default()
            .in_format(color_format.into())
            .out_format(color_format.into());

        unsafe {
            citro3d_sys::C3D_RenderTargetSetOutput(
                raw,
                screen.as_raw(),
                screen.side().into(),
                flags.bits(),
            );
        }

        Ok(Self {
            raw,
            _screen: screen,
            _queue: queue,
        })
    }

    /// Clear the render target with the given 32-bit RGBA color and depth buffer value.
    /// Use `flags` to specify whether color and/or depth should be overwritten.
    #[doc(alias = "C3D_RenderTargetClear")]
    pub fn clear(&mut self, flags: ClearFlags, rgba_color: u32, depth: u32) {
        unsafe {
            citro3d_sys::C3D_RenderTargetClear(self.raw, flags.bits(), rgba_color, depth);
        }
    }

    /// Return the underlying `citro3d` render target for this target.
    pub(crate) fn as_raw(&self) -> *mut C3D_RenderTarget {
        self.raw
    }
}

impl Drop for Target<'_> {
    #[doc(alias = "C3D_RenderTargetDelete")]
    fn drop(&mut self) {
        unsafe {
            C3D_RenderTargetDelete(self.raw);
        }
    }
}

impl From<FramebufferFormat> for ColorFormat {
    fn from(format: FramebufferFormat) -> Self {
        match format {
            FramebufferFormat::Rgba8 => Self::RGBA8,
            FramebufferFormat::Rgb565 => Self::RGB565,
            FramebufferFormat::Rgb5A1 => Self::RGBA5551,
            FramebufferFormat::Rgba4 => Self::RGBA4,
            // this one seems unusual, but it appears to work fine:
            FramebufferFormat::Bgr8 => Self::RGB8,
        }
    }
}

impl DepthFormat {
    fn as_raw(self) -> C3D_DEPTHTYPE {
        C3D_DEPTHTYPE {
            __e: self as GPU_DEPTHBUF,
        }
    }
}
