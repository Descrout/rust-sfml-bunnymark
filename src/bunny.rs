use crate::sprite_batch::TextureRegion;
use rand::Rng;

const BUNNY_SIZE: u32 = 36;

pub struct Bunny {
    pub region: TextureRegion,
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
    dx: f32,
    dy: f32,
    gravity: f32,
}

impl Bunny {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();

        let resize_rand: i32 = rng.gen_range(-12, 12);
        let w = (BUNNY_SIZE as i32 + resize_rand) as u32;
        let h = (BUNNY_SIZE as i32 + resize_rand) as u32;

        let x_rand = rng.gen_range(-100.0, 100.0);
        let dx = if rng.gen::<bool>() { 300.0 } else { -300.0 };

        let bunny_type = rng.gen_range(0, 12);

        Self {
            region: TextureRegion::new(bunny_type * BUNNY_SIZE as i32, 0, BUNNY_SIZE, BUNNY_SIZE),
            x,
            y,
            w,
            h,
            dx: dx + x_rand,
            dy: -200.0,
            gravity: rng.gen_range(1400.0, 2100.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.dx * dt;
        self.y += self.dy * dt;

        self.dy += self.gravity * dt;

        self.check_collision();

        // resurrect when they slow down vertically
        if self.dy.abs() < 100.0 && self.y > (crate::HEIGHT - 110 - self.h) as f32 {
            self.dy = -1500.0;
        }
    }

    fn check_collision(&mut self) {
        if self.x as u32 + self.w > crate::WIDTH {
            self.dx *= -1.0;
            self.x = (crate::WIDTH - self.w - 1) as f32;
        }

        if self.x < 0.0 {
            self.dx *= -1.0;
            self.x = 1.0;
        }

        if self.y as u32 + self.h > crate::HEIGHT {
            self.dy *= -0.7;
            self.y = (crate::HEIGHT - self.h - 1) as f32;
        }

        if self.y < 0.0 {
            self.dy *= -0.7;
            self.y = 1.0;
        }
    }
}
