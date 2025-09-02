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

struct Frame;

#[non_exhaustive]
#[must_use]
pub struct RenderPass<'pass> {
    texenvs: [OnceCell<TexEnv>; texenv::TEXENV_COUNT],
    _active_frame: Frame,

    // It is not valid behaviour to bind anything but a correct shader program.
    // Instead of binding NULL, we simply force the user to have a shader program bound again
    // before any draw calls.
    is_program_bound: bool,

    _phantom: PhantomData<&'pass mut Instance>,
}

impl<'pass> RenderPass<'pass> {
    pub(crate) fn new(_instance: &'pass mut Instance) -> Self {
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
            _active_frame: Frame::new(),
            is_program_bound: false,
            _phantom: PhantomData,
        }
    }

    /// Select the given render target for the following draw calls.
    ///
    /// # Errors
    ///
    /// Fails if the given target cannot be used for drawing.
    #[doc(alias = "C3D_FrameDrawOn")]
    pub fn select_render_target(&mut self, target: &'pass Target<'_>) -> Result<()> {
        let _ = self;
        if unsafe { citro3d_sys::C3D_FrameDrawOn(target.as_raw()) } {
            Ok(())
        } else {
            Err(Error::InvalidRenderTarget)
        }
    }

    /// Get the buffer info being used, if it exists.
    ///
    /// # Notes
    ///
    /// The resulting [`buffer::Info`] is copied (and not taken) from the one currently in use.
    #[doc(alias = "C3D_GetBufInfo")]
    pub fn buffer_info(&self) -> Option<buffer::Info> {
        let raw = unsafe { citro3d_sys::C3D_GetBufInfo() };
        buffer::Info::copy_from(raw)
    }

    /// Set the buffer info to use for for the following draw calls.
    #[doc(alias = "C3D_SetBufInfo")]
    pub fn set_buffer_info(&mut self, buffer_info: &buffer::Info) {
        let raw: *const _ = &buffer_info.0;
        // LIFETIME SAFETY: C3D_SetBufInfo actually copies the pointee instead of mutating it.
        unsafe { citro3d_sys::C3D_SetBufInfo(raw.cast_mut()) };
    }

    /// Get the attribute info being used, if it exists.
    ///
    /// # Notes
    ///
    /// The resulting [`attrib::Info`] is copied (and not taken) from the one currently in use.
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
    ///
    /// # Panics
    ///
    /// Panics if no shader program was bound (see [`RenderPass::bind_program`]).
    #[doc(alias = "C3D_DrawArrays")]
    pub fn draw_arrays(&mut self, primitive: buffer::Primitive, vbo_data: buffer::Slice<'pass>) {
        // TODO: Decide whether it's worth returning an `Error` instead of panicking.
        if !self.is_program_bound {
            panic!("tried todraw arrays when no shader program is bound");
        }

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

    /// Draws the vertices in `buf` indexed by `indices`.
    ///
    /// # Panics
    ///
    /// Panics if no shader program was bound (see [`RenderPass::bind_program`]).
    #[doc(alias = "C3D_DrawElements")]
    pub fn draw_elements<I: Index>(
        &mut self,
        primitive: buffer::Primitive,
        vbo_data: buffer::Slice<'pass>,
        indices: &Indices<'pass, I>,
    ) {
        if !self.is_program_bound {
            panic!("tried to draw elements when no shader program is bound");
        }

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

    /// Use the given [`shader::Program`] for the following draw calls.
    pub fn bind_program(&mut self, program: &'pass shader::Program) {
        // SAFETY: AFAICT C3D_BindProgram just copies pointers from the given program,
        // instead of mutating the pointee in any way that would cause UB
        unsafe {
            citro3d_sys::C3D_BindProgram(program.as_raw().cast_mut());
        }

        self.is_program_bound = true;
    }

    /// Binds a [`LightEnv`] for the following draw calls.
    pub fn bind_light_env(&mut self, env: Option<Pin<&'pass mut LightEnv>>) {
        unsafe {
            citro3d_sys::C3D_LightEnvBind(env.map_or(std::ptr::null_mut(), |env| env.as_raw_mut()));
        }
    }

    /// Bind a uniform to the given `index` in the vertex shader for the next draw call.
    ///
    /// # Panics
    ///
    /// Panics if no shader program was bound (see [`RenderPass::bind_program`]).
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
        if !self.is_program_bound {
            panic!("tried to bind vertex uniform when no shader program is bound");
        }

        // LIFETIME SAFETY: Uniform data is copied into global buffers.
        uniform.into().bind(self, shader::Type::Vertex, index);
    }

    /// Bind a uniform to the given `index` in the geometry shader for the next draw call.
    ///
    /// # Panics
    ///
    /// Panics if no shader program was bound (see [`RenderPass::bind_program`]).
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
        if !self.is_program_bound {
            panic!("tried to bind geometry uniform when no shader program is bound");
        }

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
    ///
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

impl Frame {
    fn new() -> Self {
        unsafe {
            citro3d_sys::C3D_FrameBegin(
                // TODO: begin + end flags should be configurable
                citro3d_sys::C3D_FRAME_SYNCDRAW,
            )
        };

        Self {}
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            citro3d_sys::C3D_FrameEnd(0);
        }
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

impl Drop for RenderPass<'_> {
    fn drop(&mut self) {
        unsafe {
            // TODO: substitute as many as possible with safe wrappers.
            // These resets are derived from the implementation of `C3D_Init` and by studying the `C3D_Context` struct.
            citro3d_sys::C3D_DepthMap(true, -1.0, 0.0);
            citro3d_sys::C3D_CullFace(ctru_sys::GPU_CULL_BACK_CCW);
            citro3d_sys::C3D_StencilTest(false, ctru_sys::GPU_ALWAYS, 0x00, 0xFF, 0x00);
            citro3d_sys::C3D_StencilOp(
                ctru_sys::GPU_STENCIL_KEEP,
                ctru_sys::GPU_STENCIL_KEEP,
                ctru_sys::GPU_STENCIL_KEEP,
            );
            citro3d_sys::C3D_BlendingColor(0);
            citro3d_sys::C3D_EarlyDepthTest(false, ctru_sys::GPU_EARLYDEPTH_GREATER, 0);
            citro3d_sys::C3D_DepthTest(true, ctru_sys::GPU_GREATER, ctru_sys::GPU_WRITE_ALL);
            citro3d_sys::C3D_AlphaTest(false, ctru_sys::GPU_ALWAYS, 0x00);
            citro3d_sys::C3D_AlphaBlend(
                ctru_sys::GPU_BLEND_ADD,
                ctru_sys::GPU_BLEND_ADD,
                ctru_sys::GPU_SRC_ALPHA,
                ctru_sys::GPU_ONE_MINUS_SRC_ALPHA,
                ctru_sys::GPU_SRC_ALPHA,
                ctru_sys::GPU_ONE_MINUS_SRC_ALPHA,
            );
            citro3d_sys::C3D_FragOpMode(ctru_sys::GPU_FRAGOPMODE_GL);
            citro3d_sys::C3D_FragOpShadow(0.0, 1.0);

            // The texCoordId has no importance since we are binding NULL
            citro3d_sys::C3D_ProcTexBind(0, std::ptr::null_mut());

            // ctx->texConfig = BIT(12); I have not found a way to replicate this one yet (maybe not necessary because of texenv's unbinding).

            // ctx->texShadow = BIT(0);
            citro3d_sys::C3D_TexShadowParams(true, 0.0);

            // ctx->texEnvBuf = 0; I have not found a way to replicate this one yet (maybe not necessary because of texenv's unbinding).

            // ctx->texEnvBufClr = 0xFFFFFFFF;
            citro3d_sys::C3D_TexEnvBufColor(0xFFFFFFFF);
            // ctx->fogClr = 0;
            citro3d_sys::C3D_FogColor(0);
            //ctx->fogLut = NULL;
            citro3d_sys::C3D_FogLutBind(std::ptr::null_mut());

            // We don't need to unbind programs (and in citro3D you can't),
            // since the user is forced to bind them again before drawing next time they render.

            self.bind_light_env(None);

            // TODO: C3D_TexBind doesn't work for NULL
            // https://github.com/devkitPro/citro3d/blob/9f21cf7b380ce6f9e01a0420f19f0763e5443ca7/source/texture.c#L222
            /*for i in 0..3 {
                citro3d_sys::C3D_TexBind(i, std::ptr::null_mut());
            }*/

            for i in 0..6 {
                self.texenv(texenv::Stage::new(i).unwrap()).reset();
            }

            // Unbind attribute information (can't use NULL pointer, so we use an empty attrib::Info instead).
            //
            // TODO: Drawing nothing actually hangs the GPU, so this code is never really helpful (also, not used since the flag makes it a non-issue).
            //       Is it worth keeping? Could hanging be considered better than an ARM exception?
            let empty_info = attrib::Info::default();
            self.set_attr_info(&empty_info);

            // ctx->fixedAttribDirty = 0;
            // ctx->fixedAttribEverDirty = 0;
            for i in 0..12 {
                let vec = citro3d_sys::C3D_FixedAttribGetWritePtr(i);
                (*vec).c = [0.0, 0.0, 0.0, 0.0];
            }
        }
    }
}
