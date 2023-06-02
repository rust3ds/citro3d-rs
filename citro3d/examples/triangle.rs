//! This example demonstrates the most basic usage of `citro3d`: rendering a simple
//! RGB triangle (sometimes called a "Hello triangle") to the 3DS screen.

#![feature(allocator_api)]

use citro3d::attrib::{self};
use citro3d::buffer::{self};
use citro3d::render::{ClearFlags, Target};
use citro3d::{include_aligned_bytes, shader};
use citro3d_sys::C3D_Mtx;
use ctru::prelude::*;
use ctru::services::gfx::{RawFrameBuffer, Screen, TopScreen3D};

use std::f32::consts::PI;
use std::ffi::CStr;
use std::mem::MaybeUninit;

#[repr(C)]
#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: Vec3,
    color: Vec3,
}

static VERTICES: &[Vertex] = &[
    Vertex {
        pos: Vec3::new(0.0, 0.5, 0.5),
        color: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, 0.5),
        color: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, 0.5),
        color: Vec3::new(0.0, 0.0, 1.0),
    },
];

static SHADER_BYTES: &[u8] =
    include_aligned_bytes!(concat!(env!("OUT_DIR"), "/examples/assets/vshader.shbin"));

fn main() {
    ctru::use_panic_handler();

    let mut soc = Soc::new().expect("failed to get SOC");
    drop(soc.redirect_to_3dslink(true, true));

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let top_screen = TopScreen3D::from(&gfx.top_screen);

    let (mut top_left, mut top_right) = top_screen.split_mut();

    let RawFrameBuffer { width, height, .. } = top_left.raw_framebuffer();
    let mut top_left_target = citro3d::render::Target::new(width, height, top_left, None)
        .expect("failed to create render target");

    let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
    let mut top_right_target = citro3d::render::Target::new(width, height, top_right, None)
        .expect("failed to create render target");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = bottom_screen.raw_framebuffer();

    let mut bottom_target = citro3d::render::Target::new(width, height, bottom_screen, None)
        .expect("failed to create bottom screen render target");

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();

    let mut program = shader::Program::new(vertex_shader).unwrap();

    let mut vbo_data = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
    vbo_data.extend_from_slice(VERTICES);

    let mut buf_info = buffer::Info::new();
    let (attr_info, vbo_idx) = prepare_vbos(&mut buf_info, &vbo_data);

    let (projection_uniform_idx, mut projection) = scene_init(&mut program);

    unsafe { citro3d_sys::Mtx_RotateY(&mut projection, -PI / 12.0, true) };

    let mut right_eye_projection = projection;
    unsafe { citro3d_sys::Mtx_RotateY(&mut right_eye_projection, 2.0 * PI / 12.0, true) };

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        instance.render_frame_with(|instance| {
            let mut render_to = |target: &mut Target, projection| {
                instance
                    .select_render_target(target)
                    .expect("failed to set render target");

                let clear_color: u32 = 0x7F_7F_7F_FF;
                target.clear(ClearFlags::ALL, clear_color, 0);

                unsafe {
                    // Update the uniforms
                    citro3d_sys::C3D_FVUnifMtx4x4(
                        ctru_sys::GPU_VERTEX_SHADER,
                        projection_uniform_idx.into(),
                        projection,
                    );
                }

                instance.set_attr_info(&attr_info);

                instance.draw_arrays(buffer::Primitive::Triangles, vbo_idx);
            };

            render_to(&mut top_left_target, &projection);
            render_to(&mut top_right_target, &right_eye_projection);
            render_to(&mut bottom_target, &projection);
        });
    }
}

// sheeeesh, this sucks to type:
fn prepare_vbos<'buf, 'info, 'vbo>(
    buf_info: &'info mut buffer::Info,
    vbo_data: &'vbo [Vertex],
) -> (attrib::Info, buffer::Slice<'buf>)
where
    'info: 'buf,
    'vbo: 'buf,
{
    // Configure attributes for use with the vertex shader
    let mut attr_info = attrib::Info::new();

    let reg0 = attrib::Register::new(0).unwrap();
    let reg1 = attrib::Register::new(1).unwrap();

    attr_info
        .add_loader(reg0, attrib::Format::Float, 3)
        .unwrap();

    attr_info
        .add_loader(reg1, attrib::Format::Float, 5)
        .unwrap();

    let buf_idx = buf_info.add(vbo_data, &attr_info).unwrap();

    (attr_info, buf_idx)
}

fn scene_init(program: &mut shader::Program) -> (i8, C3D_Mtx) {
    // Load the vertex shader, create a shader program and bind it
    unsafe {
        citro3d_sys::C3D_BindProgram(program.as_raw());

        // Get the location of the uniforms
        let projection_name = CStr::from_bytes_with_nul(b"projection\0").unwrap();
        let projection_uniform_idx = ctru_sys::shaderInstanceGetUniformLocation(
            (*program.as_raw()).vertexShader,
            projection_name.as_ptr(),
        );

        // Compute the projection matrix
        let projection = {
            let mut projection = MaybeUninit::uninit();
            citro3d_sys::Mtx_OrthoTilt(
                projection.as_mut_ptr(),
                // The 3ds top screen is a 5:3 ratio
                -1.66,
                1.66,
                -1.0,
                1.0,
                0.0,
                1.0,
                true,
            );
            projection.assume_init()
        };

        // Configure the first fragment shading substage to just pass through the vertex color
        // See https://www.opengl.org/sdk/docs/man2/xhtml/glTexEnv.xml for more insight
        let env = citro3d_sys::C3D_GetTexEnv(0);
        citro3d_sys::C3D_TexEnvInit(env);
        citro3d_sys::C3D_TexEnvSrc(
            env,
            citro3d_sys::C3D_Both,
            ctru_sys::GPU_PRIMARY_COLOR,
            0,
            0,
        );
        citro3d_sys::C3D_TexEnvFunc(env, citro3d_sys::C3D_Both, ctru_sys::GPU_REPLACE);

        (projection_uniform_idx, projection)
    }
}
