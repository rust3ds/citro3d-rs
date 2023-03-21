//! This example demonstrates the most basic usage of `citro3d`: rendering a simple
//! RGB triangle (sometimes called a "Hello triangle") to the 3DS screen.

#![feature(allocator_api)]

use citro3d::attrib::{self};
use citro3d::buffer::{self};
use citro3d::render::{ClearFlags, Target};
use citro3d::{include_aligned_bytes, shader};
use citro3d_sys::C3D_Mtx;
use ctru::gfx::{Gfx, RawFrameBuffer, Screen};
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use ctru::services::soc::Soc;

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
    let mut soc = Soc::init().expect("failed to get SOC");
    drop(soc.redirect_to_3dslink(true, true));

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let mut top_screen = gfx.top_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = top_screen.get_raw_framebuffer();

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let mut top_target = citro3d::render::Target::new(width, height, top_screen, None)
        .expect("failed to create render target");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = bottom_screen.get_raw_framebuffer();

    let mut bottom_target = citro3d::render::Target::new(width, height, bottom_screen, None)
        .expect("failed to create bottom screen render target");

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();

    let mut program = shader::Program::new(vertex_shader).unwrap();

    let mut vbo_data = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
    vbo_data.extend_from_slice(VERTICES);
    let vbo_idx = prepare_vbos(&vbo_data);

    let (uloc_projection, projection) = scene_init(&mut program);

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        let mut render_to = |target: &mut Target| {
            instance.render_frame_with(|instance| {
                instance
                    .select_render_target(target)
                    .expect("failed to set render target");

                let clear_color: u32 = 0x7F_7F_7F_FF;
                target.clear(ClearFlags::ALL, clear_color, 0);

                unsafe {
                    // Update the uniforms
                    citro3d_sys::C3D_FVUnifMtx4x4(
                        ctru_sys::GPU_VERTEX_SHADER,
                        uloc_projection.into(),
                        &projection,
                    );
                }

                instance.draw_arrays(buffer::Primitive::Triangles, vbo_idx);
            });
        };

        render_to(&mut top_target);
        render_to(&mut bottom_target);
    }
}

fn prepare_vbos(vbo_data: &[Vertex]) -> buffer::Index {
    // Configure attributes for use with the vertex shader
    let mut attr_info = attrib::Info::get_mut().expect("failed to get global attr info");

    let reg0 = attrib::Register::new(0).unwrap();
    let reg1 = attrib::Register::new(1).unwrap();

    // The default permutation would actually already be what we want if we
    // inserted position, then color, but just show that it's customizable
    // by swapping the order then using `set_permutation`.

    let color_attr = attr_info
        .add_loader(reg0, attrib::Format::Float, 3)
        .unwrap();

    let position_attr = attr_info
        .add_loader(reg1, attrib::Format::Float, 3)
        .unwrap();

    attr_info
        .set_permutation(&[position_attr, color_attr])
        .unwrap();

    // Configure buffers
    let mut buf_info = buffer::Info::get_mut().unwrap();
    let buf_idx = buf_info.add(vbo_data, &attr_info).unwrap();

    buf_idx
}

fn scene_init(program: &mut shader::Program) -> (i8, C3D_Mtx) {
    // Load the vertex shader, create a shader program and bind it
    unsafe {
        citro3d_sys::C3D_BindProgram(program.as_raw());

        // Get the location of the uniforms
        let projection_name = CStr::from_bytes_with_nul(b"projection\0").unwrap();
        let uloc_projection = ctru_sys::shaderInstanceGetUniformLocation(
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

        (uloc_projection, projection)
    }
}
