use crate::utils::{WIDTH, HEIGHT};
use raylib::prelude::*;

pub struct Brick {
    // Drawing contents
    pub rec:        Rectangle,
    pub collider:   Rectangle,
    color:      Color,

    // Physics contents
    rotation:   f32,
    origin:     Vector2,
    velocity:   Vector2,
    mass:       f32
}

impl Brick {
    const FRESH:    Color   = Color::new(0xc6, 0x42, 0x42, 0xff);
    const BROKEN:   Color   = Color::new(0x82, 0x3f, 0x40, 0xff); 


    pub fn new() -> Self {
        let rec    = Rectangle::new(0.0, 0.0, 50.0, 50.0);
        let origin = Vector2::new(rec.width / 2.0, rec.height / 2.0);


        Self {
            rec:        rec,
            collider:   Rectangle::new(0.0, HEIGHT - origin.y, WIDTH, 0.0),
            color:      Self::FRESH,

            rotation:   0.0,
            origin:     origin,
            velocity:   Vector2::new(0.0, 0.0),
            mass:       0.2
        }
    }

    pub fn drop(&mut self, mouse_pos: Vector2) {
        self.rec.x = mouse_pos.x;
        self.rec.y = mouse_pos.y;
    }

    pub fn update(&mut self, dt: f32) {
        // Falling logic
        Self::fall(self, dt);

        self.rec.x += self.velocity.x * dt;
        self.rec.y += self.velocity.y * dt;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let thick   = self.rec.width * 0.2;
        let body = Rectangle::new(
            self.rec.x,
            self.rec.y,
            self.rec.width  - thick,
            self.rec.height - thick
        );

        d.draw_rectangle_pro(
            self.rec,
            self.origin,
            self.rotation,
            Color::BLACK
        );

        d.draw_rectangle_pro(
            body,
            Vector2::new(
                body.width  * 0.5,
                body.height * 0.5,
            ),
            self.rotation,
            self.color
        );

    }

    fn fall(&mut self, dt: f32) {
        // TODO:
        // - add collider for detecting another rectangles 
        let gravity = 982.0;

        
        self.velocity.y += if self.rec.y > self.collider.y {
            if self.velocity.y > 200.0 {
                self.color = Self::BROKEN;
            }

            self.rec.y = self.collider.y;
            self.velocity.y = -self.velocity.y * self.mass;


            0.0
        }

        else {
            gravity * dt
        };
    }
}
