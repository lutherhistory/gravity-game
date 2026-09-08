use raylib::prelude::*;

const WIDTH:  i32 = 800;
const HEIGHT: i32 = 500;

fn main() {
    // Raylib Initialization
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Gravity")
        .build();

    rl.set_target_fps(60);

    // Entity Works
    let mut rect = Rectangle {
        x: (WIDTH as f32 - 50.0) / 2.0,
        y: 100.0,
        width: 50.0,
        height: 50.0
    };
    let mut velocity: f32 = 0.0;
    let mut on_floor: bool = false;
    let gravity = 200.0;

    while !rl.window_should_close() {
        // Updating
        let dt = rl.get_frame_time();

        if !on_floor {
            velocity += gravity * dt;
            if rect.y > HEIGHT as f32 - rect.height {
                on_floor = true;
                velocity = 0.0;
                rect.y   = HEIGHT as f32 - rect.height;
            }
        }

        rect.y += velocity * dt;

        // Drawing
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_rectangle_rec(
            rect,
            Color::BLUE
        );
    }
}
