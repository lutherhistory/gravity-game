use raylib::prelude::*;

mod blocks;
use blocks::Brick;

mod utils;
use utils::{WIDTH, HEIGHT};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Aesthetic Castle")
        .build();

    rl.set_target_fps(60);

    let count = 300;

    let mut bricks: Vec<Brick>  = Vec::new();
    let mut in_used: usize      = 0;


    while !rl.window_should_close() {
        let dt          = rl.get_frame_time();
        let mouse_pos   = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if in_used < count {
                bricks.push(Brick::new());
                bricks[in_used].drop(mouse_pos);
                in_used += 1;
            }
        }

        for i in 0..in_used {
            bricks[i].update(dt);
        }


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        for i in 0..in_used {
            bricks[i].draw(&mut d);
        }

        d.draw_text(&((count - in_used).to_string() + "/" + &count.to_string()), 0, 0, 30, Color::BLACK);
    }
}
