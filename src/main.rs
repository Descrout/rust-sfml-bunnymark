extern crate rand;
extern crate sfml;

mod bunny;
mod sprite_batch;

use bunny::Bunny;
use sprite_batch::SpriteBatch;
use std::time::Instant;

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{mouse, Event, Style},
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

struct Game {
    window: RenderWindow,
    batch: SpriteBatch,
    bunnies: Vec<Bunny>,
    info_text: String,
}

impl Game {
    fn new() -> Self {
        let window = RenderWindow::new(
            (WIDTH, HEIGHT),
            "Right click to add more bunnies !",
            Style::CLOSE,
            &Default::default(),
        );
        Self {
            window,
            batch: SpriteBatch::new("lineup.png"),
            bunnies: Vec::with_capacity(100000),
            info_text: String::new(),
        }
    }

    fn add_bunnies(&mut self, count: usize) {
        let mouse = self.window.mouse_position();
        for _ in 0..count {
            self.bunnies
                .push(Bunny::new(mouse.x as f32, mouse.y as f32));
        }
    }

    fn update(&mut self, dt: f32) {
        if self.window.has_focus() && mouse::Button::Right.is_pressed() {
            self.add_bunnies(10);
        }

        for bunny in self.bunnies.iter_mut() {
            bunny.update(dt);
            
            self.batch
                .add(bunny.x, bunny.y, bunny.w, bunny.h, bunny.region);
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::BLACK);
        self.batch.display(&self.window);
        self.window.display();
    }

    fn run(&mut self) {
        let mut delta_time = 0.0;

        //Debug info vars
        let mut timer = 0.0;
        let mut frames = 0u32;

        //Start main loop
        loop {
            let start = Instant::now();
            while let Some(event) = self.window.poll_event() {
                if event == Event::Closed {
                    return;
                }
            }
            self.update(delta_time);
            self.draw();

            //Calculate FPS
            timer += delta_time;
            frames += 1;
            if timer >= 1.0 {
                self.info_text = format!("Bunnies : {}, FPS : {} - Right click to add more bunnies !", self.bunnies.len(), frames);
                self.window.set_title(&self.info_text);
                frames = 0;
                timer = 0.0;
            }
            
            //Fps Lock
            let elapsed = start.elapsed().as_millis() as i32;
            let wait_ms = if elapsed >= 16 { 16 } else { 16 - elapsed };
            sfml::system::sleep(sfml::system::Time::milliseconds(wait_ms));
            delta_time = start.elapsed().as_secs_f32();
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.add_bunnies(100);
    game.run();
}
