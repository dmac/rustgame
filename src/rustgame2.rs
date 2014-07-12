extern crate native;
extern crate rsfml;
extern crate time;

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Texture, Sprite, Color, Font, Text};

struct Entity<'a> {
    x: i64,
    y: i64,
    x_rem: i64,
    y_rem: i64,
    speed: u64, // pixels per second
    sprite: Sprite<'a>,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl<'a> Entity<'a> {
    fn move(&mut self, direction: Direction, dt: u64) {
        let distance = (self.speed * dt / 1000000000) as i64;
        let remainder = (self.speed * dt % 1000000000) as i64;
        match direction {
            North => {
                self.y -= distance;
                self.y_rem -= remainder;
            }
            East => {
                self.x += distance;
                self.x_rem += remainder;
            }
            South => {
                self.y += distance;
                self.y_rem += remainder;
            }
            West => {
                self.x -= distance;
                self.x_rem -= remainder;
            }
        }
        let extra_y = self.y_rem / 1000000000;
        if extra_y > 1 || extra_y < -1 {
            self.y += extra_y;
            self.y_rem = self.y_rem % 1000000000;
        }
        let extra_x = self.x_rem / 1000000000;
        if extra_x > 1 || extra_x < -1 {
            self.x += extra_x;
            self.x_rem = self.x_rem % 1000000000;
        }
    }

    fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x as f32, self.y as f32);
        w.draw(&self.sprite);
    }
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main () -> () {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "SFML Example",
                                       Close,
                                       &ContextSettings::default())
        .expect("error creating window");
    window.set_framerate_limit(60);

    let texture = Texture::new_from_file("resources/link.gif").expect("error loading texture");
    let sprite = Sprite::new_with_texture(&texture).expect("error creating sprite");

    let font = Font::new_from_file("resources/Inconsolata-Regular.ttf").expect("error loading font");

    let mut entity = Entity{
        x: 50,
        y: 100,
        x_rem: 0,
        y_rem: 0,
        speed: 100,
        sprite: sprite,
    };

    let mut last_time = time::precise_time_ns();
    let mut fps_last_time = last_time;
    let mut fps_count = 0u;
    let mut fps_text = Text::new_init("", &font, 48).expect("error creating text");
    while window.is_open() {
        fps_count += 1;
        let curr_time = time::precise_time_ns();
        let dt = curr_time - last_time;
        last_time = curr_time;
        if curr_time - fps_last_time > 1000000000 {
            fps_text.set_string(fps_count.to_string().as_slice());
            fps_last_time = curr_time;
            fps_count = 0;
        }

        if keyboard::is_key_pressed(keyboard::Up) { entity.move(North, dt) }
        if keyboard::is_key_pressed(keyboard::Right) { entity.move(East, dt) }
        if keyboard::is_key_pressed(keyboard::Down) { entity.move(South, dt) }
        if keyboard::is_key_pressed(keyboard::Left) { entity.move(West, dt) }
        for event in window.events() {
            match event {
                event::KeyPressed{ code: keyboard::Escape, .. } |
                event::Closed => window.close(),
                event::KeyPressed{ code: code, .. } => {
                    // println!("{}", code);
                }
                _ => {}
            }
        }

        window.clear(&Color::new_RGB(50, 50, 50));
        window.draw(&fps_text);
        entity.draw(&mut window);
        window.display()
    }
}
