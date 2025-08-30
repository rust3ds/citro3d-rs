#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]
#![doc(html_root_url = "https://rust3ds.github.io/citro2d-rs/crates")]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]

//! Safe Rust bindings to `citro2d`. This crate wraps `citro2d-sys` to provide
//! safer APIs for graphics programs targeting the 3DS.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]

pub mod error;
pub mod render;
pub mod shapes;
use citro2d_sys::C2D_DEFAULT_MAX_OBJECTS;
pub use error::{Error, Result};
use render::Target;

/// The single instance for using `citro2d`. This is the base type that an application
/// should instantiate to use this library.
#[non_exhaustive]
#[must_use]
pub struct Instance {
    pub citro3d_instance: citro3d::Instance,
}

impl Instance {
    /// Create a new instance of `citro2d`.
    /// This also initializes `citro3d` since it is required for `citro2d`.
    pub fn new() -> Result<Self> {
        let citro3d_instance = citro3d::Instance::new().expect("failed to initialize Citro3D");
        let citro2d = Self::with_max_objects(
            C2D_DEFAULT_MAX_OBJECTS.try_into().unwrap(),
            citro3d_instance,
        );

        citro2d
    }

    /// You have to initialize citro3d before using citro2d, but some cases you may
    /// Have initialized citro3d already, so you can use this function to initialize
    /// You pass in the citro3d instance you already initialized to ensure it's lifetime is the same as citro2d
    /// **Note** The above statement may not work, and may not be able to switch between the two without api changes
    /// but currently working on that assumption and to allow for flexibility for the developer
    pub fn new_without_c3d_init(citro3d_instance: citro3d::Instance) -> Result<Self> {
        Self::with_max_objects(
            C2D_DEFAULT_MAX_OBJECTS.try_into().unwrap(),
            citro3d_instance,
        )
    }

    /// Create a new instance of `citro2d` with a custom maximum number of objects.
    #[doc(alias = "C2D_Init")]
    #[doc(alias = "C2D_Prepare")]
    pub fn with_max_objects(
        max_objects: usize,
        citro3d_instance: citro3d::Instance,
    ) -> Result<Self> {
        let new_citro_2d = match unsafe { citro2d_sys::C2D_Init(max_objects) } {
            true => Ok(Self {
                citro3d_instance: citro3d_instance,
            }),
            false => Err(Error::FailedToInitialize),
        };
        unsafe { citro2d_sys::C2D_Prepare() };
        new_citro_2d
    }

    /// Render 2d graphics to a selected [Target]
    #[doc(alias = "C3D_FrameBegin")]
    #[doc(alias = "C2D_SceneBegin")]
    #[doc(alias = "C3D_FrameEnd")]
    pub fn render_target<F>(&mut self, target: &mut Target<'_>, f: F)
    where
        F: FnOnce(&Self, &mut Target<'_>),
    {
        unsafe {
            citro3d_sys::C3D_FrameBegin(citro3d_sys::C3D_FRAME_SYNCDRAW);
            citro2d_sys::C2D_SceneBegin(target.raw);
            f(self, target);
            citro3d_sys::C3D_FrameEnd(0);
        }
    }

    /// Returns some stats about the 3Ds's graphics
    /// TODO this may be more appropriate in citro3d
    pub fn get_3d_stats(&self) -> Citro3DStats {
        //TODO should i check for NaN?
        let processing_time_f32 = unsafe { citro3d_sys::C3D_GetProcessingTime() };
        let drawing_time_f32 = unsafe { citro3d_sys::C3D_GetDrawingTime() };
        let cmd_buf_usage_f32 = unsafe { citro3d_sys::C3D_GetCmdBufUsage() };
        Citro3DStats {
            processing_time: processing_time_f32,
            drawing_time: drawing_time_f32,
            cmd_buf_usage: cmd_buf_usage_f32,
        }
    }
}

/// Stats about the 3Ds's graphics
pub struct Citro3DStats {
    pub processing_time: f32,
    pub drawing_time: f32,
    pub cmd_buf_usage: f32,
}

/// A 2D point in space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn new_no_z(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y, z: 0.0 }
    }
}

/// Size of a 2D object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}
