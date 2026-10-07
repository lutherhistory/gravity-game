use raylib::prelude::*;


const WIDTH:    f32 = 800.0;
const HEIGHT:   f32 = 500.0;


struct Block {
    rec: Rectangle,
    color: Color,

    // Physics
    velocity: Vector2,
}

impl Block {
    pub fn new(c: Color) -> Self {
        let ran_value_x = unsafe { raylib::ffi::GetRandomValue(0, (WIDTH - 50.0)  as i32) as f32 };
        let ran_value_y = unsafe { raylib::ffi::GetRandomValue(0, (HEIGHT - 50.0) as i32) as f32 };


        Self {
            rec: Rectangle::new(
                ran_value_x, 
                ran_value_y, 
                50.0, 
                50.0
            ),
            color: c,
            velocity: Vector2::new(100.0, 100.0)
        }
    }

    pub fn fall(&mut self, dt: f32) -> bool {
//        if self.rec.y < HEIGHT - self.rec.height {
//            self.velocity.y += 10.0;
//        }
//
//        else {
//            self.velocity.y = 0.0;
//        }
//
//        self.rec.y += self.velocity.y * dt;

        let mut status: bool = false;

        if self.rec.x < 0.0 || self.rec.x > WIDTH - self.rec.width {
            self.velocity.x *= - 1.0;
            status = true;
        };

        if self.rec.y < 0.0 || self.rec.y > HEIGHT - self.rec.height {
            self.velocity.y *= - 1.0;
            status = true;
        }


        self.rec.x += self.velocity.x * dt;
        self.rec.y += self.velocity.y * dt;

        status
    }
}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Aesthetic Castle")
        .build();
    rl.set_target_fps(60);


    let mut blocks: Vec<Block> = vec![];

    blocks.push(Block::new(Color::BLUE));
    blocks.push(Block::new(Color::RED));
    blocks.push(Block::new(Color::YELLOW));
    blocks.push(Block::new(Color::GREEN));
    blocks.push(Block::new(Color::PINK));


    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        for block in &mut blocks {
            if block.fall(dt) {
                //blocks.push(Block::new(Color::GREEN));
            }

            d.draw_rectangle_rec(block.rec, block.color);
        }
    }
}
