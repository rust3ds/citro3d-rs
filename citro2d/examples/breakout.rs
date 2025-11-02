//! This example demonstrates a simple 2d game of the classic game Breakout
//! Very simple implementation with bugs, but to show case a simple 2d game
#![feature(allocator_api)]

use citro2d::render::{Color, Target};
use citro2d::shapes::{CircleSolid, RectangleSolid};
use citro2d::{Point, Size};
use ctru::{prelude::*, services::gfx::TopScreen3D};

const TOP_SCREEN_WIDTH: u16 = 400;
const TOP_SCREEN_HEIGHT: u16 = 240;

const BOTTOM_SCREEN_WIDTH: u16 = 320;
const BOTTOM_SCREEN_HEIGHT: u16 = 240;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut citro2d_instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (top_left, _) = top_screen.split_mut();
    let mut top_target = Target::new(top_left).expect("failed to create render target");

    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    let white = Color::new(255, 255, 255);
    let black = Color::new(0, 0, 0);

    let mut paddle = Paddle {
        position: Point::new(
            BOTTOM_SCREEN_WIDTH as f32 / 2.0,
            BOTTOM_SCREEN_HEIGHT as f32 - 10.0,
            0.0,
        ),
        size: (75.0, 10.0).into(),
        color: white,
    };

    let mut ball = Ball {
        position: Point::new(
            BOTTOM_SCREEN_WIDTH as f32 / 2.0,
            (BOTTOM_SCREEN_HEIGHT - 15) as f32,
            0.0,
        ),
        radius: 5.0,
        color: white,
        velocity: Point::new(2.0, -2.0, 0.0),
    };
    let collors_of_rainbow = [
        Color::new(255, 0, 0),
        Color::new(255, 127, 0),
        Color::new(255, 255, 0),
        Color::new(0, 255, 0),
        Color::new(0, 0, 255),
        Color::new(75, 0, 130),
        Color::new(148, 0, 211),
    ];
    let mut bricks = Vec::new();
    for row in 0..7 {
        for column in 0..12 {
            bricks.push(Brick {
                position: Point::new(column as f32 * 32.0, row as f32 * 16.0, 0.0),
                size: (30.0, 15.0).into(),
                color: collors_of_rainbow[row],
                is_alive: true,
            });
        }
    }

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        if hid.keys_held().contains(KeyPad::LEFT) || hid.keys_held().contains(KeyPad::DPAD_LEFT) {
            paddle.move_left();
        }

        if hid.keys_held().contains(KeyPad::RIGHT) || hid.keys_held().contains(KeyPad::DPAD_RIGHT) {
            paddle.move_right();
        }

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| {
            render_target.clear(black);

            paddle.render(render_target);

            ball.bounce(&paddle);
            for brick in &mut bricks {
                if brick.is_alive {
                    brick.live_or_die(&mut ball);
                    brick.render(render_target);
                }
            }
            //circles are better to render last for performance reasons
            ball.render(render_target);
        });

        let stats = citro2d_instance.get_3d_stats();
        bottom_screen.select();
        println!("\x1b[1;1HSimple Rusty citro2d shapes example");
        println!("\x1b[2;1HCPU: {:6.2}%", stats.processing_time * 6.0);
        println!("\x1b[3;1HGPU: {:6.2}%", stats.drawing_time * 6.0);
        println!("\x1b[4;1HCmdBuf: {:6.2}%", stats.cmd_buf_usage * 100.0);

        //Uncomment to cap fps
        // gfx.wait_for_vblank();
    }
}

struct Paddle {
    pub position: Point,
    pub size: Size,
    pub color: Color,
}

impl Paddle {
    fn render(&self, render_target: &mut Target) {
        render_target.render_2d_shape(&RectangleSolid {
            point: self.position,
            size: self.size,
            color: self.color,
        });
    }

    fn move_left(&mut self) {
        if self.position.x > 0.0 {
            self.position.x -= 2.0;
        }
    }

    fn move_right(&mut self) {
        if self.position.x <= BOTTOM_SCREEN_WIDTH as f32 {
            self.position.x += 2.0;
        }
    }
}

struct Ball {
    pub position: Point,
    pub radius: f32,
    pub color: Color,
    pub velocity: Point,
}

impl Ball {
    fn render(&self, render_target: &mut Target) {
        render_target.render_2d_shape(&CircleSolid {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            radius: self.radius,
            color: self.color,
        });
    }

    fn bounce(&mut self, paddle: &Paddle) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // Check for collision with the walls
        if self.position.x - self.radius <= 0.0
            || self.position.x + self.radius >= TOP_SCREEN_WIDTH as f32
        {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y - self.radius <= 0.0 {
            self.velocity.y = -self.velocity.y;
        }

        // Check for collision with the paddle
        if self.position.y + self.radius >= paddle.position.y
            && self.position.x >= paddle.position.x
            && self.position.x <= paddle.position.x + paddle.size.width
        {
            self.velocity.y = -self.velocity.y;
        }

        // Check if the ball hits the bottom of the screen
        if self.position.y + self.radius >= TOP_SCREEN_WIDTH as f32 {
            // Reset the ball
            self.position = Point::new(
                TOP_SCREEN_WIDTH as f32 / 2.0,
                TOP_SCREEN_HEIGHT as f32 / 2.0,
                0.0,
            );
            self.velocity = Point::new(2.0, -2.0, 0.0);
        }
    }
}

struct Brick {
    pub position: Point,
    pub size: Size,
    pub color: Color,
    pub is_alive: bool,
}

impl Brick {
    fn render(&self, render_target: &mut Target) {
        if self.is_alive {
            render_target.render_2d_shape(&RectangleSolid {
                point: self.position,
                size: self.size,
                color: self.color,
            });
        }
    }

    fn check_collision(&self, ball: &mut Ball) -> bool {
        let brick_left = self.position.x;
        let brick_right = self.position.x + self.size.width;
        let brick_top = self.position.y;
        let brick_bottom = self.position.y + self.size.height;

        let ball_left = ball.position.x - ball.radius;
        let ball_right = ball.position.x + ball.radius;
        let ball_top = ball.position.y - ball.radius;
        let ball_bottom = ball.position.y + ball.radius;

        if ball_left < brick_right
            && ball_right > brick_left
            && ball_top < brick_bottom
            && ball_bottom > brick_top
        {
            // Determine the side of the collision and bounce the ball accordingly
            if ball.velocity.x > 0.0 && ball_left < brick_right && ball_right > brick_left {
                ball.velocity.x = -ball.velocity.x;
            } else if ball.velocity.x < 0.0 && ball_right > brick_left && ball_left < brick_right {
                ball.velocity.x = -ball.velocity.x;
            }

            if ball.velocity.y > 0.0 && ball_top < brick_bottom && ball_bottom > brick_top {
                ball.velocity.y = -ball.velocity.y;
            } else if ball.velocity.y < 0.0 && ball_bottom > brick_top && ball_top < brick_bottom {
                ball.velocity.y = -ball.velocity.y;
            }

            return true;
        }

        false
    }

    fn live_or_die(&mut self, ball: &mut Ball) {
        if self.check_collision(ball) {
            self.is_alive = false;
        }
    }
}
