#![feature(allocator_api)]
use std::f32::consts::PI;

use citro3d::{
    attrib, buffer,
    color::Color,
    light::{DistanceAttenuation, LightEnv, Lut, LutId, LutInput, Material, Spotlight},
    math::{AspectRatio, ClipPlanes, FVec3, Matrix4, Projection, StereoDisplacement},
    render::{self, ClearFlags},
    shader, texenv,
};
use citro3d_macros::include_shader;
use ctru::services::{
    apt::Apt,
    gfx::{Gfx, RawFrameBuffer, Screen, TopScreen3D},
    hid::{Hid, KeyPad},
};

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
#[derive(Copy, Clone)]
#[repr(C)]
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
    normal: Vec3,
    uv: Vec2,
}

impl Vertex {
    const fn new(pos: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self { pos, normal, uv }
    }
}

static SHADER_BYTES: &[u8] = include_shader!("assets/frag-shader.pica");

const VERTICES: &[Vertex] = &[
    Vertex::new(
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.0, -1.0, 0.0),
        Vec2::new(0.0, 0.0),
    ),
];

fn main() {
    ctru::set_panic_hook(true);

    let gfx = Gfx::with_formats_shared(
        ctru::services::gspgpu::FramebufferFormat::Rgba8,
        ctru::services::gspgpu::FramebufferFormat::Rgba8,
    )
    .expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let top_screen = TopScreen3D::from(&gfx.top_screen);

    let (mut top_left, mut top_right) = top_screen.split_mut();

    let RawFrameBuffer { width, height, .. } = top_left.raw_framebuffer();
    let mut top_left_target = instance
        .render_target(
            width,
            height,
            top_left,
            Some(render::DepthFormat::Depth24Stencil8),
        )
        .expect("failed to create render target");

    let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
    let mut top_right_target = instance
        .render_target(
            width,
            height,
            top_right,
            Some(render::DepthFormat::Depth24Stencil8),
        )
        .expect("failed to create render target");

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = bottom_screen.raw_framebuffer();

    let mut bottom_target = instance
        .render_target(
            width,
            height,
            bottom_screen,
            Some(render::DepthFormat::Depth24Stencil8),
        )
        .expect("failed to create bottom screen render target");

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();

    let program = shader::Program::new(vertex_shader).unwrap();
    instance.bind_program(&program);

    let mut vbo_data = Vec::with_capacity_in(VERTICES.len(), ctru::linear::LinearAllocator);
    vbo_data.extend_from_slice(VERTICES);
    let mut buf_info = buffer::Info::new();
    let (attr_info, vbo_data) = prepare_vbos(&mut buf_info, &vbo_data);

    // Setup the global lighting environment, using an exponential lookup-table.
    let mut light_env = LightEnv::new_pinned();
    light_env.as_mut().connect_lut(
        LutId::D0,
        LutInput::LightNormal,
        Lut::from_fn(|v| v.powf(20.0), false),
    );
    light_env.as_mut().set_material(Material {
        ambient: Some(Color::new(0.2, 0.2, 0.2)),
        diffuse: Some(Color::new(1.0, 0.4, 1.0)),
        specular0: Some(Color::new(0.8, 0.8, 0.8)),
        ..Default::default()
    });

    // Create a new light instance.
    let light = light_env.as_mut().create_light().unwrap();
    let mut light = light_env.as_mut().light_mut(light).unwrap();
    light.as_mut().set_color(Color::new(1.0, 1.0, 1.0)); // White color
    light.as_mut().set_position(FVec3::new(0.0, 0.0, -1.0)); // Approximately emitting from the camera
    // Set how the light attenuates over distance.
    // This particular LUT is optimized to work between 0 and 10 units of distance from the light point.
    light
        .as_mut()
        .set_distance_attenutation(Some(DistanceAttenuation::new(0.0..10.0, |d| {
            (1.0 / (2.0 * PI * d * d)).min(1.0)
        })));

    // Subtle spotlight pointed at the top of the cube.
    let light = light_env.as_mut().create_light().unwrap();
    let mut light = light_env.as_mut().light_mut(light).unwrap();
    light.as_mut().set_color(Color::new(0.5, 0.5, 0.5));
    light
        .as_mut()
        .set_spotlight(Some(Spotlight::with_cutoff(PI / 8.0))); // Spotlight angle of PI/6
    light
        .as_mut()
        .set_spotlight_direction(FVec3::new(0.0, 0.4, -1.0)); // Slightly tilted upwards
    light
        .as_mut()
        .set_distance_attenutation(Some(DistanceAttenuation::new(0.0..10.0, |d| {
            (1.0 / (0.5 * PI * d * d)).min(1.0) // We use a less aggressive attenuation to highlight the spotlight
        })));

    // Bind the lighting environment for use
    instance.bind_light_env(Some(light_env));

    // Setup the rotating view of the cube
    let mut view = Matrix4::identity();
    let model_idx = program.get_uniform("modelView").unwrap();
    view.translate(0.0, 0.0, -2.0);
    instance.bind_vertex_uniform(model_idx, view);

    let stage0 = texenv::Stage::new(0).unwrap();
    instance
        .texenv(stage0)
        .src(
            texenv::Mode::BOTH,
            texenv::Source::FragmentPrimaryColor,
            Some(texenv::Source::FragmentSecondaryColor),
            None,
        )
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Add);

    let projection_uniform_idx = program.get_uniform("projection").unwrap();

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        instance.render_frame_with(|instance| {
            let mut render_to = |target: &mut render::Target, projection| {
                target.clear(ClearFlags::ALL, 0, 0);

                instance
                    .select_render_target(target)
                    .expect("failed to set render target");

                instance.bind_vertex_uniform(projection_uniform_idx, projection);
                instance.bind_vertex_uniform(model_idx, view);

                instance.set_attr_info(&attr_info);

                instance.draw_arrays(buffer::Primitive::Triangles, vbo_data);
            };

            let Projections {
                left_eye,
                right_eye,
                center,
            } = calculate_projections();

            render_to(&mut top_left_target, &left_eye);
            render_to(&mut top_right_target, &right_eye);
            render_to(&mut bottom_target, &center);
        });

        // Rotate the modelView
        view.translate(0.0, 0.0, 2.0);
        view.rotate_y(1.0f32.to_radians());
        view.translate(0.0, 0.0, -2.0);
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
    let reg2 = attrib::Register::new(2).unwrap();

    attr_info
        .add_loader(reg0, attrib::Format::Float, 3)
        .unwrap();

    attr_info
        .add_loader(reg1, attrib::Format::Float, 3)
        .unwrap();

    attr_info
        .add_loader(reg2, attrib::Format::Float, 2)
        .unwrap();

    let buf_idx = buf_info.add(vbo_data, &attr_info).unwrap();

    (attr_info, buf_idx)
}

struct Projections {
    left_eye: Matrix4,
    right_eye: Matrix4,
    center: Matrix4,
}

fn calculate_projections() -> Projections {
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
