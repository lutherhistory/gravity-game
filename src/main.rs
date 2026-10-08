use raylib::prelude::*;

const WIDTH:    f32 = 800.0;
const HEIGHT:   f32 = 500.0;

const BLOCKS_COUNT: usize = 100;

const BLUE_BOX: Color  = Color::new(0x42, 0x92, 0xc6, 0xff);
const RED_BOX : Color  = Color::new(0xc6, 0x42, 0x42, 0xff);
const BG: Color = Color::new(0xa6, 0xa2, 0xa2, 0xff);


struct Block {
    rec: Rectangle,
    color: Color,

    velocity: Vector2
}

impl Block {
    pub fn new(c: Color) -> Self {
        //let rx = unsafe { raylib::ffi::GetRandomValue(0, (WIDTH - 50.0) as i32) } as f32;
        let rx = 0.0;
        let ry = 0.0;


        Self {
            rec: Rectangle::new(rx, ry, 50.0, 50.0),
            color: c,
            velocity: Vector2::new(0.0, 0.0)
        }
    }

    pub fn fall(&mut self, dt: f32) {
        self.velocity.y = if self.rec.y < HEIGHT - self.rec.height {
            self.velocity.y + 100.0
        } 

        else {
            self.rec.y = HEIGHT - self.rec.height;
            0.0
        };

        
        self.rec.y += self.velocity.y * dt;
        self.rec.x += self.velocity.x * dt;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Aesthetic Castle")
        .build();
    rl.set_target_fps(60);


    let mut blocks: Vec<Block> = Vec::new();
    let mut in_used: usize = 0;


    for _i in 0..BLOCKS_COUNT {
        blocks.push(Block::new(RED_BOX));
    }

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        // --- Update Logics ---
        {
            in_used += if in_used < BLOCKS_COUNT && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                blocks[in_used].rec.x = mouse_pos.x - blocks[in_used].rec.width / 2.0;
                blocks[in_used].rec.y = mouse_pos.y;
                1
            }

            else {
                0
            }
        }

        // --- Drawing ---
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG);
        {
            d.draw_text(&((blocks.len() - in_used).to_string() + "/" + &BLOCKS_COUNT.to_string()), 0, 0, 30, Color::BLACK);

            for i in 0..in_used {
//                for j in 0..in_used {
//                    if i != j && blocks[i].rec.check_collision_recs(blocks[j].rec) {
//                        let overlap = blocks[i].rec.get_collision_rec(blocks[j].rec).unwrap();
//
//                        if overlap.width > overlap.height {
//                            blocks[j].rec.y += overlap.width;
//                        }
//                    }
//                }

                blocks[i].fall(dt);
                d.draw_rectangle_rec(blocks[i].rec, blocks[i].color);
            }

            let line_start  = Vector2::new(mouse_pos.x, mouse_pos.y);
            let line_end    = Vector2::new(mouse_pos.x, HEIGHT);


            d.draw_line_dashed(
                line_start,
                line_end,
                20,
                10,
                Color::GOLD
            );

        }
    }
}
