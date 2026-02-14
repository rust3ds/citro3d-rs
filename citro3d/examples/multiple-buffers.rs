//! This example demonstrates the most basic usage of `citro3d`: rendering a simple
//! RGB triangle (sometimes called a "Hello triangle") to the 3DS screen.

#![feature(allocator_api)]

use citro3d::macros::include_shader;
use citro3d::math::{AspectRatio, ClipPlanes, Matrix4, Projection, StereoDisplacement};
use citro3d::render::{ClearFlags, Frame, ScreenTarget, Target};
use citro3d::texenv;
use citro3d::{attrib, buffer, shader};
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

static VERTEX_POSITIONS: &[Vec3] = &[
    Vec3::new(0.0, 0.5, -3.0),
    Vec3::new(-0.5, -0.5, -3.0),
    Vec3::new(0.5, -0.5, -3.0),
];

static VERTEX_COLS: &[Vec3] = &[
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
];

static SHADER_BYTES: &[u8] = include_shader!("assets/vshader.pica");
const CLEAR_COLOR: u32 = 0x68_B0_D8_FF;

fn main() {
    let mut soc = Soc::new().expect("failed to get SOC");
    drop(soc.redirect_to_3dslink(true, true));

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let top_screen = TopScreen3D::from(&gfx.top_screen);

    let (mut top_left, mut top_right) = top_screen.split_mut();

    let RawFrameBuffer { width, height, .. } = top_left.raw_framebuffer();
    let mut top_left_target = instance
        .render_target(width, height, top_left, None)
        .expect("failed to create render target");

    let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
    let mut top_right_target = instance
        .render_target(width, height, top_right, None)
        .expect("failed to create render target");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = bottom_screen.raw_framebuffer();

    let mut bottom_target = instance
        .render_target(width, height, bottom_screen, None)
        .expect("failed to create bottom screen render target");

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();

    let program = shader::Program::new(vertex_shader).unwrap();
    let projection_uniform_idx = program.get_uniform("projection").unwrap();

    let vbo_pos = buffer::Buffer::new(VERTEX_POSITIONS);
    let vbo_col = buffer::Buffer::new(VERTEX_COLS);

    let mut buf_info = buffer::Info::new();
    let attr_info = prepare_vbos(&mut buf_info, vbo_pos, vbo_col);

    let stage0 = texenv::TexEnv::new()
        .src(texenv::Mode::BOTH, texenv::Source::PrimaryColor, None, None)
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Replace);

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        instance.render_frame_with(|mut frame| {
            // Sadly closures can't have lifetime specifiers,
            // so we wrap `render_to` in this function to force the borrow checker rules.
            fn cast_lifetime_to_closure<'frame, T>(x: T) -> T
            where
                T: Fn(&mut Frame<'frame>, &'frame mut ScreenTarget<'_>, &Matrix4),
            {
                x
            }

            let render_to = cast_lifetime_to_closure(|frame, target, projection| {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);

                frame
                    .select_render_target(target)
                    .expect("failed to set render target");
                frame.bind_vertex_uniform(projection_uniform_idx, projection);

                frame.set_texenvs(&[stage0]);

                frame.set_attr_info(&attr_info);

                frame
                    .draw_arrays(buffer::Primitive::Triangles, &buf_info, None)
                    .unwrap();
            });

            // We bind the vertex shader.
            frame.bind_program(&program);

            // Configure the first fragment shading substage to just pass through the vertex color
            // See https://www.opengl.org/sdk/docs/man2/xhtml/glTexEnv.xml for more insight

            let Projections {
                left_eye,
                right_eye,
                center,
            } = calculate_projections();

            render_to(&mut frame, &mut top_left_target, &left_eye);
            render_to(&mut frame, &mut top_right_target, &right_eye);
            render_to(&mut frame, &mut bottom_target, &center);

            frame
        });
    }
}

fn prepare_vbos<'a>(
    buf_info: &'a mut buffer::Info,
    positions: buffer::Buffer,
    cols: buffer::Buffer,
) -> attrib::Info {
    use attrib::{Format, Info, Permutation, Register};

    const REG_POS: Register = Register::V0;
    const REG_COL: Register = Register::V1;

    // Configure attributes for use with the vertex shader
    let mut attr_info = Info::new();

    attr_info.add_loader(REG_POS, Format::Float, 3).unwrap();

    attr_info.add_loader(REG_COL, Format::Float, 3).unwrap();

    buf_info
        .add(positions, Permutation::from_layout(&[REG_POS]).unwrap())
        .unwrap();
    buf_info
        .add(cols, Permutation::from_layout(&[REG_COL]).unwrap())
        .unwrap();

    attr_info
}

struct Projections {
    left_eye: Matrix4,
    right_eye: Matrix4,
    center: Matrix4,
}

fn calculate_projections() -> Projections {
    // TODO: it would be cool to allow playing around with these parameters on
    // the fly with D-pad, etc.
    let slider_val = ctru::os::current_3d_slider_state();
    let interocular_distance = slider_val / 2.0;

    let vertical_fov = 40.0_f32.to_radians();
    let screen_depth = 2.0;

    let clip_planes = ClipPlanes {
        near: 0.01,
        far: 100.0,
    };

    let (left, right) = StereoDisplacement::new(interocular_distance, screen_depth);

    let (left_eye, right_eye) =
        Projection::perspective(vertical_fov, AspectRatio::TopScreen, clip_planes)
            .stereo_matrices(left, right);

    let center =
        Projection::perspective(vertical_fov, AspectRatio::BottomScreen, clip_planes).into();

    Projections {
        left_eye,
        right_eye,
        center,
    }
}
