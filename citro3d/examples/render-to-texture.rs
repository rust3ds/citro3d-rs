#![feature(allocator_api)]

use citro3d::macros::include_shader;
use citro3d::math::{AspectRatio, ClipPlanes, Matrix4, Projection, StereoDisplacement};
use citro3d::render::{ClearFlags, Target};
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
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: Vec3,
    tex_coord: Vec2,
}

static VERTICES: &[Vertex] = &[
    Vertex {
        pos: Vec3::new(-0.5, 0.5, -3.0),
        tex_coord: Vec2::new(0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, -0.5, -3.0),
        tex_coord: Vec2::new(0.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, -3.0),
        tex_coord: Vec2::new(1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(-0.5, 0.5, -3.0),
        tex_coord: Vec2::new(0.0, 1.0),
    },
    Vertex {
        pos: Vec3::new(0.5, -0.5, -3.0),
        tex_coord: Vec2::new(1.0, 0.0),
    },
    Vertex {
        pos: Vec3::new(0.5, 0.5, -3.0),
        tex_coord: Vec2::new(1.0, 1.0),
    },
];

static SHADER_BYTES: &[u8] = include_shader!("assets/vshader_textured.pica");
static TEXTURE_BYTES: &[u8] = include_bytes!("assets/kitten.t3d");
const CLEAR_COLOR: u32 = 0x68_B0_D8_FF;
const OTHER_CLEAR_COLOR: u32 = 0xFF_FF_FF_FF - 0x68_B0_D8_00;
// The screen framebuffer is in format BRG8 while the texture is RGB8, so we swap the B and G components (and shift over to ignore alpha)
const OTHER_COLOR_BGR8: u32 = (OTHER_CLEAR_COLOR & 0x00_FF_00_00 >> 8)
    | (OTHER_CLEAR_COLOR & 0x00_00_FF_00 << 8)
    | (OTHER_CLEAR_COLOR & 0xFF_00_00_00 >> 24);

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

    let mut vbo_data = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
    vbo_data.extend_from_slice(VERTICES);

    let mut buf_info = buffer::Info::new();
    let (attr_info, vbo_data) = prepare_vbos(&mut buf_info, &vbo_data);

    let mut tex_target = instance
        .render_target_texture(create_texture_target(), texture::Face::default(), None)
        .unwrap();
    let tex_kitten = create_texture_kitten();

    let stage0 = texenv::TexEnv::new()
        .src(texenv::Mode::BOTH, texenv::Source::Texture0, None, None)
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Replace);

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        instance.render_frame_with(|mut frame| {
            let Projections {
                left_eye,
                right_eye,
                center,
            } = calculate_projections();
            let square = calculate_projection_square();

            // We bind the vertex shader.
            frame.bind_program(&program);
            frame.set_attr_info(&attr_info);
            frame.set_texenvs(&[stage0]);

            // Bottom screen
            bottom_target.clear(ClearFlags::ALL, OTHER_COLOR_BGR8, 0);
            frame.bind_texture(texture::Index::Texture0, &tex_kitten);
            frame
                .select_render_target(&bottom_target)
                .expect("failed to set render target");
            frame.bind_vertex_uniform(projection_uniform_idx, center);
            frame.draw_arrays(buffer::Primitive::Triangles, vbo_data);

            // Render to texture
            tex_target.clear(ClearFlags::ALL, OTHER_CLEAR_COLOR, 0);
            frame.bind_texture(texture::Index::Texture0, &tex_kitten);
            frame
                .select_render_target(&tex_target)
                .expect("failed to set render target");
            frame.bind_vertex_uniform(projection_uniform_idx, square);
            frame.draw_arrays(buffer::Primitive::Triangles, vbo_data);

            // Render texture to top screen targets
            for (target, proj) in [
                (&mut top_left_target, left_eye),
                (&mut top_right_target, right_eye),
            ] {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);
                frame
                    .select_render_target(target)
                    .expect("failed to set render target");
                frame.bind_texture(texture::Index::Texture0, tex_target.texture());
                frame.bind_vertex_uniform(projection_uniform_idx, proj);
                frame.draw_arrays(buffer::Primitive::Triangles, vbo_data);
            }

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
    let reg1 = attrib::Register::new(1).unwrap();

    attr_info
        .add_loader(reg0, attrib::Format::Float, 3)
        .unwrap();

    attr_info
        .add_loader(reg1, attrib::Format::Float, 2)
        .unwrap();

    let buf_idx = buf_info.add(vbo_data, &attr_info).unwrap();

    (attr_info, buf_idx)
}

struct Projections {
    left_eye: Matrix4,
    right_eye: Matrix4,
    center: Matrix4,
}

fn create_texture_target() -> texture::Texture {
    // Texture targets have to be in vram
    let params = texture::TextureParameters::new_2d_in_vram(64, 64, texture::ColorFormat::Rgb8);
    let mut tex = texture::Texture::new(params).unwrap();
    tex.set_filter(texture::Filter::Linear, texture::Filter::Nearest);
    tex
}

fn create_texture_kitten() -> texture::Texture {
    let tex: texture::Tex3DSTexture = texture::Tex3DSTexture::new(TEXTURE_BYTES, false).unwrap();
    let mut tex: texture::Texture = tex.into_texture();
    tex.set_filter(texture::Filter::Linear, texture::Filter::Linear);
    tex
}

fn calculate_projection_square() -> Matrix4 {
    let vertical_fov = 40.0_f32.to_radians();
    let clip_planes = ClipPlanes {
        near: 0.01,
        far: 100.0,
    };

    Projection::perspective(vertical_fov, AspectRatio::Square, clip_planes).into()
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
