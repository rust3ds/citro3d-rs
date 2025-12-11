//! This example demonstrates the most basic usage of `citro3d`: rendering a simple
//! RGB triangle (sometimes called a "Hello triangle") to the 3DS screen.

#![feature(allocator_api)]

use citro3d::macros::include_shader;
use citro3d::math::{AspectRatio, ClipPlanes, Matrix4, Projection, StereoDisplacement};
use citro3d::render::{self, ClearFlags, Frame, ScreenTarget, Target};
use citro3d::{attrib, buffer, shader};
use citro3d::{texenv, texture};
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
}

impl Vertex {
    const fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            pos: Vec3::new(x, y, z),
        }
    }
}

static VERTICES: &[Vertex] = &[
    // First face (PZ)
    // First triangle
    Vertex::new(-0.5, -0.5, 0.5),
    Vertex::new(0.5, -0.5, 0.5),
    Vertex::new(0.5, 0.5, 0.5),
    // Second triangle
    Vertex::new(0.5, 0.5, 0.5),
    Vertex::new(-0.5, 0.5, 0.5),
    Vertex::new(-0.5, -0.5, 0.5),
    // Second face (MZ)
    // First triangle
    Vertex::new(-0.5, -0.5, -0.5),
    Vertex::new(-0.5, 0.5, -0.5),
    Vertex::new(0.5, 0.5, -0.5),
    // Second triangle
    Vertex::new(0.5, 0.5, -0.5),
    Vertex::new(0.5, -0.5, -0.5),
    Vertex::new(-0.5, -0.5, -0.5),
    // Third face (PX)
    // First triangle
    Vertex::new(0.5, -0.5, -0.5),
    Vertex::new(0.5, 0.5, -0.5),
    Vertex::new(0.5, 0.5, 0.5),
    // Second triangle
    Vertex::new(0.5, 0.5, 0.5),
    Vertex::new(0.5, -0.5, 0.5),
    Vertex::new(0.5, -0.5, -0.5),
    // Fourth face (MX)
    // First triangle
    Vertex::new(-0.5, -0.5, -0.5),
    Vertex::new(-0.5, -0.5, 0.5),
    Vertex::new(-0.5, 0.5, 0.5),
    // Second triangle
    Vertex::new(-0.5, 0.5, 0.5),
    Vertex::new(-0.5, 0.5, -0.5),
    Vertex::new(-0.5, -0.5, -0.5),
    // Fifth face (PY)
    // First triangle
    Vertex::new(-0.5, 0.5, -0.5),
    Vertex::new(-0.5, 0.5, 0.5),
    Vertex::new(0.5, 0.5, 0.5),
    // Second triangle
    Vertex::new(0.5, 0.5, 0.5),
    Vertex::new(0.5, 0.5, -0.5),
    Vertex::new(-0.5, 0.5, -0.5),
    // Sixth face (MY)
    // First triangle
    Vertex::new(-0.5, -0.5, -0.5),
    Vertex::new(0.5, -0.5, -0.5),
    Vertex::new(0.5, -0.5, 0.5),
    // Second triangle
    Vertex::new(0.5, -0.5, 0.5),
    Vertex::new(-0.5, -0.5, 0.5),
    Vertex::new(-0.5, -0.5, -0.5),
];

static SHADER_BYTES: &[u8] = include_shader!("assets/vshader_skybox.pica");
static TEXTURE_BYTES: &[u8] = include_bytes!("assets/skybox.t3d");
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
    let model_view_uniform_idx = program.get_uniform("modelView").unwrap();

    let mut vbo_data = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
    vbo_data.extend_from_slice(VERTICES);

    let mut buf_info = buffer::Info::new();
    let (attr_info, vbo_data) = prepare_vbos(&mut buf_info, &vbo_data);

    let tex = create_texture();

    let stage0 = texenv::TexEnv::new()
        .src(texenv::Mode::BOTH, texenv::Source::Texture0, None, None)
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Replace);

    let mut angle_x: f32 = 0.0;
    let mut angle_y: f32 = 0.0;

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Controls to look around
        let mut circle_x = hid.circlepad_position().0 as f32 / 80.0f32;
        if circle_x.abs() < 0.2 {
            circle_x = 0.0;
        }
        let mut circle_y = hid.circlepad_position().1 as f32 / 80.0f32;
        if circle_y.abs() < 0.2 {
            circle_y = 0.0;
        }
        angle_x -= circle_y * 2.0f32.to_radians();
        if hid.keys_held().contains(KeyPad::DPAD_DOWN) {
            angle_x += 2.0f32.to_radians();
        }
        if angle_x > 90.0f32.to_radians() {
            angle_x = 90.0f32.to_radians();
        }
        if hid.keys_held().contains(KeyPad::DPAD_UP) {
            angle_x -= 2.0f32.to_radians();
        }
        if angle_x < -90.0f32.to_radians() {
            angle_x = -90.0f32.to_radians();
        }
        angle_y += circle_x * 2.0f32.to_radians();
        if hid.keys_held().contains(KeyPad::DPAD_RIGHT) {
            angle_y += 2.0f32.to_radians();
        }
        if hid.keys_held().contains(KeyPad::DPAD_LEFT) {
            angle_y -= 2.0f32.to_radians();
        }

        instance.render_frame_with(|mut frame| {
            let mut model_view = Matrix4::identity();
            model_view.rotate_y(angle_y);
            model_view.rotate_x(angle_x);

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
                frame.bind_vertex_uniform(model_view_uniform_idx, model_view);
                frame.draw_arrays(buffer::Primitive::Triangles, vbo_data);
            });

            // We bind the vertex shader.
            frame.bind_program(&program);
            frame.set_attr_info(&attr_info);
            frame.set_cull_face(render::effect::CullMode::FrontCounterClockwise);
            frame.set_texenvs(&[stage0]);
            frame.bind_texture(texture::Index::Texture0, &tex);

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
    vbo_data: &'a [Vertex],
) -> (attrib::Info, buffer::Slice<'a>) {
    // Configure attributes for use with the vertex shader
    let mut attr_info = attrib::Info::new();

    let reg0 = attrib::Register::new(0).unwrap();

    attr_info
        .add_loader(reg0, attrib::Format::Float, 3)
        .unwrap();

    let buf_idx = buf_info.add(vbo_data, &attr_info).unwrap();

    (attr_info, buf_idx)
}

struct Projections {
    left_eye: Matrix4,
    right_eye: Matrix4,
    center: Matrix4,
}

fn create_texture() -> texture::Texture {
    let tex: texture::Tex3DSTexture = texture::Tex3DSTexture::new(TEXTURE_BYTES, false).unwrap();
    let mut tex: texture::Texture = tex.into_texture();
    tex.set_filter(texture::Filter::Linear, texture::Filter::Linear);
    tex.set_wrap(texture::Wrap::ClampToEdge, texture::Wrap::ClampToEdge);
    tex
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
