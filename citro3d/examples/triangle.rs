//! This example demonstrates the most basic usage of `citro3d`: rendering a simple
//! RGB triangle (sometimes called a "Hello triangle") to the 3DS screen.

#![feature(allocator_api)]

use citro3d::macros::include_shader;
use citro3d::math::{ClipPlane, CoordinateSystem, Matrix, Orientation, Stereoscopic};
use citro3d::render::{self, ClearFlags};
use citro3d::{attrib, buffer, shader, AspectRatio};
use ctru::prelude::*;
use ctru::services::gfx::{RawFrameBuffer, Screen, TopScreen3D};

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
        pos: Vec3::new(0.0, 0.5, 3.0),
        color: Vec3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, 3.0),
        color: Vec3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, 3.0),
        color: Vec3::new(0.0, 0.0, 1.0),
    },
];

static SHADER_BYTES: &[u8] = include_shader!("assets/vshader.pica");

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
    let mut top_left_target =
        render::Target::new(width, height, top_left, None).expect("failed to create render target");

    let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
    let mut top_right_target = render::Target::new(width, height, top_right, None)
        .expect("failed to create render target");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = bottom_screen.raw_framebuffer();

    let mut bottom_target = render::Target::new(width, height, bottom_screen, None)
        .expect("failed to create bottom screen render target");

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();

    let mut program = shader::Program::new(vertex_shader).unwrap();

    let mut vbo_data = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
    vbo_data.extend_from_slice(VERTICES);

    let mut buf_info = buffer::Info::new();
    let (attr_info, vbo_idx) = prepare_vbos(&mut buf_info, &vbo_data);

    scene_init(&mut program);

    let projection_uniform_idx = program.get_uniform("projection").unwrap();

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        instance.render_frame_with(|instance| {
            let mut render_to = |target: &mut render::Target, projection| {
                instance
                    .select_render_target(target)
                    .expect("failed to set render target");

                let clear_color: u32 = 0x7F_7F_7F_FF;
                target.clear(ClearFlags::ALL, clear_color, 0);

                instance.bind_vertex_uniform(projection_uniform_idx, projection);

                instance.set_attr_info(&attr_info);

                instance.draw_arrays(buffer::Primitive::Triangles, vbo_idx);
            };

            let Projections {
                left_eye: left,
                right_eye: right,
                center,
            } = calculate_projections();

            render_to(&mut top_left_target, &left);
            render_to(&mut top_right_target, &right);
            render_to(&mut bottom_target, &center);
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
        .add_loader(reg1, attrib::Format::Float, 3)
        .unwrap();

    let buf_idx = buf_info.add(vbo_data, &attr_info).unwrap();

    (attr_info, buf_idx)
}

struct Projections {
    left_eye: Matrix,
    right_eye: Matrix,
    center: Matrix,
}

fn calculate_projections() -> Projections {
    // TODO: it would be cool to allow playing around with these parameters on
    // the fly with D-pad, etc.
    let slider_val = unsafe { ctru_sys::osGet3DSliderState() };
    let interocular_distance = slider_val / 4.0;

    let vertical_fov = 40.0_f32.to_radians();
    let screen_depth = 2.0;

    let clip_plane = ClipPlane {
        near: 0.01,
        far: 100.0,
    };

    let stereoscopic = Stereoscopic::Stereo {
        interocular_distance,
        screen_depth,
    };

    let left_eye = Matrix::perspective_projection(
        vertical_fov,
        AspectRatio::TopScreen,
        Orientation::Natural,
        clip_plane,
        stereoscopic,
        CoordinateSystem::LeftHanded,
    );

    let right_eye = Matrix::perspective_projection(
        vertical_fov,
        AspectRatio::TopScreen,
        Orientation::Natural,
        clip_plane,
        stereoscopic.invert(),
        CoordinateSystem::LeftHanded,
    );

    let center = Matrix::perspective_projection(
        vertical_fov,
        AspectRatio::BottomScreen,
        Orientation::Natural,
        clip_plane,
        Stereoscopic::Mono,
        CoordinateSystem::LeftHanded,
    );

    Projections {
        left_eye,
        right_eye,
        center,
    }
}

fn scene_init(program: &mut shader::Program) {
    // Load the vertex shader, create a shader program and bind it
    unsafe {
        citro3d_sys::C3D_BindProgram(program.as_raw_mut());

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
    }
}
