extern crate sfml;
extern crate rand;

mod sprite_batch;
mod bunny;

use std::time::{Instant, Duration};
use sprite_batch::SpriteBatch;
use bunny::Bunny;

use sfml::{
    graphics::{Color, RenderWindow, RenderTarget, Font, Text},
    window::{Event, mouse, Style},
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const FIXED_TIMESTEP: f32 = 0.016;

struct Game{
    window: RenderWindow,
    batch: SpriteBatch,
    bunnies: Vec<Bunny>,
    one_click: bool,
    fps_text: String,
}

impl Game{
    fn new() -> Self {
        let window = RenderWindow::new(
            (WIDTH, HEIGHT),
            "SFML Bunny Mark Benchmark",
            Style::CLOSE,
            &Default::default(),
        );
        Self{
            window,
            batch: SpriteBatch::new("lineup.png"),
            bunnies: Vec::new(),
            one_click: false,
            fps_text: String::new(),
        }
    }

    fn add_bunnies(&mut self, count: usize) {
        for _ in 0..count {
            self.bunnies.push(Bunny::new());
        }
    }

    fn update(&mut self) {
        if mouse::Button::Left.is_pressed(){
            if !self.one_click {
                self.one_click = true;
                self.add_bunnies(100);
            }
        }else {self.one_click = false}

        for bunny in self.bunnies.iter_mut() {
            bunny.update();
        }
        
    }

    fn draw(&mut self, text: &Text) {
        for bunny in self.bunnies.iter() {
            self.batch.add(bunny.x, bunny.y, bunny.w, bunny.h, bunny.region);
        }

        self.window.clear(Color::BLACK);
        self.batch.display(&self.window);
        self.window.draw(text);
        self.window.display();
    }

    fn run(&mut self) {
        let mut accumulator = 0.0;
        let mut delta_time = 0.0;

        //Debug info vars
        let mut timer = 0.0;
        let mut frames = 0u32;
        let font = Font::from_file("roboto.ttf").unwrap();
        let mut text = Text::new(&self.fps_text, &font, 24);
        text.set_fill_color(Color::WHITE);

        //Start main loop
        loop {
            let start = Instant::now();
            while let Some(event) = self.window.poll_event() {
                if event == Event::Closed {return}
            }

            //Fixed timestep.
            accumulator += delta_time;
            while accumulator >= FIXED_TIMESTEP {
                self.update();
                self.draw(&text);
                accumulator -= FIXED_TIMESTEP;
            }
            
            //Calculate FPS
            timer += delta_time;
            frames += 1;
            if timer >= 1.0 {
                self.fps_text = format!("Bunnies : {}, FPS : {}", self.bunnies.len(), frames);
                text.set_string(&self.fps_text);
                frames = 0;
                timer = 0.0;
            }
            
            //FPS Lock
            let elapsed = start.elapsed().as_millis() as u64;
            let wait_ms = if elapsed >= 16 {16}else{16-elapsed};
            ::std::thread::sleep(Duration::from_millis(wait_ms));
            delta_time = start.elapsed().as_secs_f32();

        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
