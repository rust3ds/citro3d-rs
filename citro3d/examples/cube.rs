//! This example demonstrates drawing a coloured cube using indexed rendering.

#![feature(allocator_api)]

use citro3d::macros::include_shader;
use citro3d::math::{
    AspectRatio, ClipPlanes, CoordinateOrientation, FVec3, Matrix4, Projection, StereoDisplacement,
};
use citro3d::render::{ClearFlags, RenderPass, Target};
use citro3d::{attrib, buffer, shader, texenv};
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

// borrowed from https://bevyengine.org/examples/3D%20Rendering/generate-custom-mesh/
const VERTS: &[[f32; 3]] = &[
    // top (facing towards +y)
    [-0.5, 0.5, -0.5], // vertex with index 0
    [0.5, 0.5, -0.5],  // vertex with index 1
    [0.5, 0.5, 0.5],   // etc. until 23
    [-0.5, 0.5, 0.5],
    // bottom   (-y)
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [-0.5, -0.5, 0.5],
    // right    (+x)
    [0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
    [0.5, 0.5, -0.5],
    // left     (-x)
    [-0.5, -0.5, -0.5],
    [-0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [-0.5, 0.5, -0.5],
    // back     (+z)
    [-0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [0.5, 0.5, 0.5],
    [0.5, -0.5, 0.5],
    // forward  (-z)
    [-0.5, -0.5, -0.5],
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, -0.5],
    [0.5, -0.5, -0.5],
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

    let mut vbo_data = Vec::with_capacity_in(VERTS.len(), ctru::linear::LinearAllocator);
    for vert in VERTS.iter().enumerate().map(|(i, v)| Vertex {
        pos: Vec3 {
            x: v[0],
            y: v[1],
            z: v[2],
        },
        color: {
            // Give each vertex a slightly different color just to highlight edges/corners
            let value = i as f32 / VERTS.len() as f32;
            Vec3::new(1.0, 0.7 * value, 0.5)
        },
    }) {
        vbo_data.push(vert);
    }

    let attr_info = build_attrib_info();

    let mut buf_info = buffer::Info::new();
    let vbo_slice = buf_info.add(&vbo_data, &attr_info).unwrap();

    let projection_uniform_idx = program.get_uniform("projection").unwrap();
    let camera_transform = Matrix4::looking_at(
        FVec3::new(1.8, 1.8, 1.8),
        FVec3::new(0.0, 0.0, 0.0),
        FVec3::new(0.0, 1.0, 0.0),
        CoordinateOrientation::RightHanded,
    );
    let indices: &[u8] = &[
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ];
    let index_buffer = vbo_slice.index_buffer(indices).unwrap();

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        instance.render_frame_with(|mut pass| {
            fn cast_lifetime_to_closure<'pass, T>(x: T) -> T
            where
                T: Fn(&mut RenderPass<'pass>, &'pass mut Target<'_>, &Matrix4),
            {
                x
            }

            let render_to = cast_lifetime_to_closure(|pass, target, projection| {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);

                pass.select_render_target(target)
                    .expect("failed to set render target");

                pass.bind_vertex_uniform(projection_uniform_idx, projection * camera_transform);

                pass.set_attr_info(&attr_info);

                pass.draw_elements(buffer::Primitive::Triangles, vbo_slice, &index_buffer);
            });

            pass.bind_program(&program);

            let stage0 = texenv::Stage::new(0).unwrap();
            pass.texenv(stage0)
                .src(texenv::Mode::BOTH, texenv::Source::PrimaryColor, None, None)
                .func(texenv::Mode::BOTH, texenv::CombineFunc::Replace);

            let Projections {
                left_eye,
                right_eye,
                center,
            } = calculate_projections();

            render_to(&mut pass, &mut top_left_target, &left_eye);
            render_to(&mut pass, &mut top_right_target, &right_eye);
            render_to(&mut pass, &mut bottom_target, &center);

            pass
        });
    }
}

fn build_attrib_info() -> attrib::Info {
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
