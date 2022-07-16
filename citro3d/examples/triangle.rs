use citro3d::{include_aligned_bytes, shader};
use citro3d_sys::C3D_Mtx;
use ctru::gfx::{Gfx, Side};
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use ctru::services::soc::Soc;

use citro3d::render::{ClearFlags, DepthFormat};

use std::ffi::CStr;
use std::mem::MaybeUninit;

#[repr(C)]
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
struct Vertex {
    pos: Vec3,
    color: Vec3,
}

const VERTICES: &[Vertex] = &[
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

const SHADER_BYTES: &[u8] =
    include_aligned_bytes!(concat!(env!("OUT_DIR"), "/examples/assets/vshader.shbin"));

fn main() {
    ctru::init();

    let mut soc = Soc::init().expect("failed to get SOC");
    drop(soc.redirect_to_3dslink(true, true));

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let mut top_screen = gfx.top_screen.borrow_mut();
    let frame_buffer = top_screen.get_raw_framebuffer(Side::Left);

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let mut render_target = citro3d::render::Target::new(
        frame_buffer.width,
        frame_buffer.height,
        &mut *top_screen,
        DepthFormat::Depth24Stencil8,
    )
    .expect("failed to create render target");

    render_target.set_output(Side::Left);

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();

    let mut program = shader::Program::new(vertex_shader).unwrap();

    let (uloc_projection, projection, vbo_data) = scene_init(&mut program);

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        instance.render_frame_with(|instance| {
            let clear_color: u32 = 0x7F_7F_7F_FF;
            render_target.clear(ClearFlags::ALL, clear_color, 0);

            instance
                .select_render_target(&render_target)
                .expect("failed to set render target");

            scene_render(uloc_projection.into(), &projection);
        });
    }

    scene_exit(vbo_data);
}

fn scene_init(program: &mut shader::Program) -> (i8, C3D_Mtx, *mut libc::c_void) {
    // Load the vertex shader, create a shader program and bind it
    unsafe {
        citro3d_sys::C3D_BindProgram(program.as_raw());

        // Get the location of the uniforms
        let projection_name = CStr::from_bytes_with_nul(b"projection\0").unwrap();
        let uloc_projection = citro3d_sys::shaderInstanceGetUniformLocation(
            (*program.as_raw()).vertexShader,
            projection_name.as_ptr(),
        );

        // Configure attributes for use with the vertex shader
        let attr_info = citro3d_sys::C3D_GetAttrInfo();
        citro3d_sys::AttrInfo_Init(attr_info);
        citro3d_sys::AttrInfo_AddLoader(attr_info, 0, citro3d_sys::GPU_FLOAT, 3); // v0=position
        citro3d_sys::AttrInfo_AddLoader(attr_info, 1, citro3d_sys::GPU_FLOAT, 3); // v1=color

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

        // Create the vertex buffer object
        let vbo_data: *mut Vertex = citro3d_sys::linearAlloc(
            std::mem::size_of_val(&VERTICES)
                .try_into()
                .expect("size fits in u32"),
        )
        .cast();

        vbo_data.copy_from(VERTICES.as_ptr(), VERTICES.len());

        // Configure buffers
        let buf_info = citro3d_sys::C3D_GetBufInfo();
        citro3d_sys::BufInfo_Init(buf_info);
        citro3d_sys::BufInfo_Add(
            buf_info,
            vbo_data.cast(),
            std::mem::size_of::<Vertex>()
                .try_into()
                .expect("size of vec3 fits in u32"),
            2,    // Each vertex has two attributes
            0x10, // v0 = position, v1 = color, in LSB->MSB nibble order
        );

        // Configure the first fragment shading substage to just pass through the vertex color
        // See https://www.opengl.org/sdk/docs/man2/xhtml/glTexEnv.xml for more insight
        let env = citro3d_sys::C3D_GetTexEnv(0);
        citro3d_sys::C3D_TexEnvInit(env);
        citro3d_sys::C3D_TexEnvSrc(
            env,
            citro3d_sys::C3D_Both,
            citro3d_sys::GPU_PRIMARY_COLOR,
            0,
            0,
        );
        citro3d_sys::C3D_TexEnvFunc(env, citro3d_sys::C3D_Both, citro3d_sys::GPU_REPLACE);

        (uloc_projection, projection, vbo_data.cast())
    }
}

fn scene_render(uloc_projection: i32, projection: &C3D_Mtx) {
    unsafe {
        // Update the uniforms
        citro3d_sys::C3D_FVUnifMtx4x4(citro3d_sys::GPU_VERTEX_SHADER, uloc_projection, projection);

        // Draw the VBO
        citro3d_sys::C3D_DrawArrays(
            citro3d_sys::GPU_TRIANGLES,
            0,
            VERTICES
                .len()
                .try_into()
                .expect("VERTICES.len() fits in i32"),
        );
    }
}

fn scene_exit(vbo_data: *mut libc::c_void) {
    unsafe {
        citro3d_sys::linearFree(vbo_data);
    }
}
