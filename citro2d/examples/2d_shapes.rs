//! This example demonstrates the most basic usage of `citro2d`: rendering shapes
//! on the top screen of the 3DS.
//! This is an exact copy of 2d_shapes from the devkitPro examples, but in Rust.
//! https://github.com/devkitPro/3ds-examples/blob/master/graphics/gpu/2d_shapes/source/main.c
#![feature(allocator_api)]

use citro2d::Point;
use citro2d::render::{Color, Target};
use citro2d::shapes::{Circle, CircleSolid, Ellipse, MultiColor, Rectangle, Triangle};
use ctru::{prelude::*, services::gfx::TopScreen3D};

const SCREEN_WIDTH: u16 = 400;
const SCREEN_HEIGHT: u16 = 240;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut citro2d_instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (top_left, _) = top_screen.split_mut();
    let mut top_target = Target::new(top_left).expect("failed to create render target");

    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());
    let clr_white = Color::new(255, 255, 255);
    let clr_green = Color::new(0, 255, 0);
    let clr_red = Color::new(255, 0, 0);
    let clr_blue = Color::new(0, 0, 255);
    let clr_circle1 = Color::new(255, 0, 255);
    let clr_circle2 = Color::new(255, 255, 0);
    let clr_circle3 = Color::new(0, 255, 255);
    let clr_solid_circle = Color::new(104, 176, 216);
    let clr_tri1 = Color::new(255, 21, 0);
    let clr_tri2 = Color::new(39, 105, 229);
    let clr_rec1 = Color::new(154, 108, 185);
    let clr_rec2 = Color::new(255, 255, 44);
    let clr_rec3 = Color::new(216, 246, 15);
    let clr_rec4 = Color::new(64, 234, 135);
    let clr_clear = Color::new_with_alpha(255, 216, 176, 104);

    while apt.main_loop() {
        hid.scan_input();

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| {
            render_target.clear(clr_clear);

            render_target.render_2d_shape(&Triangle {
                top: (25.0, 190.0).into(),
                top_color: clr_white,
                left: (0.0, SCREEN_HEIGHT as f32).into(),
                left_color: clr_tri1,
                right: (50.0, SCREEN_HEIGHT as f32).into(),
                right_color: clr_tri2,
                depth: 0.0,
            });

            render_target.render_2d_shape(&Rectangle {
                point: Point::new(350.0, 0.0, 0.0),
                size: (50.0, 50.0).into(),
                multi_color: MultiColor {
                    top_left: clr_rec1,
                    top_right: clr_rec2,
                    bottom_left: clr_rec3,
                    bottom_right: clr_rec4,
                },
            });

            // Circles require a state change (an expensive operation) within citro2d's internals, so draw them last.
            // Although it is possible to draw them in the middle of drawing non-circular objects
            // (sprites, images, triangles, rectangles, etc.) this is not recommended. They should either
            // be drawn before all non-circular objects, or afterwards.

            render_target.render_2d_shape(&Ellipse {
                point: Point::new(0.0, 0.0, 0.0),
                size: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                multi_color: MultiColor {
                    top_left: clr_circle1,
                    top_right: clr_circle2,
                    bottom_left: clr_circle3,
                    bottom_right: clr_white,
                },
            });

            render_target.render_2d_shape(&Circle {
                point: Point::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32, 0.0),
                radius: 50.0,
                multi_color: MultiColor {
                    top_left: clr_circle3,
                    top_right: clr_white,
                    bottom_left: clr_circle1,
                    bottom_right: clr_circle2,
                },
            });

            render_target.render_2d_shape(&Circle {
                point: Point::new(25.0, 25.0, 0.0),
                radius: 25.0,
                multi_color: MultiColor {
                    top_left: clr_red,
                    top_right: clr_blue,
                    bottom_left: clr_green,
                    bottom_right: clr_white,
                },
            });

            render_target.render_2d_shape(&CircleSolid {
                x: (SCREEN_WIDTH - 25) as f32,
                y: (SCREEN_HEIGHT - 25) as f32,
                z: 0.0,
                radius: 25.0,
                color: clr_solid_circle,
            });
        });

        let stats = citro2d_instance.get_3d_stats();
        bottom_screen.select();
        println!("\x1b[1;1HSimple Rusty citro2d shapes example");
        println!("\x1b[2;1HCPU: {:6.2}%", stats.processing_time * 6.0);
        println!("\x1b[3;1HGPU: {:6.2}%", stats.drawing_time * 6.0);
        println!("\x1b[4;1HCmdBuf: {:6.2}%", stats.cmd_buf_usage * 100.0);

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Uncomment to cap fps
        // gfx.wait_for_vblank();
    }
}
