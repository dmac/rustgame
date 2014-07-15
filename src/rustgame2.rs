extern crate native;
extern crate rsfml;
extern crate time;

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Texture, Sprite, Color, Font, Text, FloatRect};

struct Entity<'a> {
    x: f32,
    y: f32,
    speed: f32, // pixels per second
    sprite: Sprite<'a>,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl<'a> Entity<'a> {
    fn move(&mut self, direction: Direction, dt: u64, world: &World) {
        let distance = self.speed * dt as f32 / 1000000000.;
        match direction {
            North => self.y -= distance,
            East => self.x += distance,
            South => self.y += distance,
            West => self.x -= distance,
        }

        let aabb = FloatRect::new(self.x, self.y,
                                  self.sprite.get_local_bounds().width,
                                  self.sprite.get_local_bounds().height);
        for block in world.blocks.iter() {
            let block_aabb = FloatRect::new(block.x,block.y,
                                            block.sprite.get_local_bounds().width,
                                            block.sprite.get_local_bounds().height);
            if FloatRect::intersects(&aabb, &block_aabb, &FloatRect::new(0.,0.,0.,0.)) {
                let mut new_x = self.x as i32;
                let mut new_y = self.y as i32;
                match direction {
                    North => {
                        while new_y < block.y as i32 + block.sprite.get_local_bounds().height as i32 {
                            new_y += 1;
                        }
                    }
                    East => {
                        while new_x + self.sprite.get_local_bounds().width as i32 > block.x as i32 {
                            new_x -= 1;
                        }
                    }
                    South => {
                        while new_y + self.sprite.get_local_bounds().height as i32 > block.y as i32 {
                            new_y -= 1;
                        }
                    }
                    West => {
                        while new_x < block.x as i32 + block.sprite.get_local_bounds().width as i32 {
                            new_x += 1;
                        }
                    }
                }
                self.x = new_x as f32;
                self.y = new_y as f32;
            }
        }
    }

    fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x, self.y);
        w.draw(&self.sprite);
    }
}

struct Block<'a> {
    x: f32,
    y: f32,
    sprite: Sprite<'a>,
}

impl<'a> std::fmt::Show for Block<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Block{{ x: {}, y: {}}}", self.x, self.y)
    }
}

struct World<'a> {
    blocks: Vec<Block<'a>>,
}

impl<'a> World<'a> {
    fn draw(&mut self, w: &mut RenderWindow) {
        for block in self.blocks.mut_iter() {
            block.sprite.set_position2f(block.x, block.y);
            w.draw(&block.sprite);
        }
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

    let player_texture = Texture::new_from_file("resources/link.gif").expect("error loading texture");
    let block_texture = Texture::new_from_file("resources/block.gif").expect("error loading texture");
    let player_sprite = Sprite::new_with_texture(&player_texture).expect("error creating sprite");
    let block_sprite = Sprite::new_with_texture(&block_texture).expect("error creating sprite");

    let font = Font::new_from_file("resources/Inconsolata-Regular.ttf").expect("error loading font");

    let mut entity = Entity{
        x: 50.,
        y: 100.,
        speed: 200.,
        sprite: player_sprite,
    };

    let mut world = World{
        blocks: Vec::new(),
    };

    for i in range(0u, 10) {
        world.blocks.push(Block{ x: (100 + i*32) as f32, y: 50., sprite: block_sprite.clone() });
        world.blocks.push(Block{ x: (100 + i*32) as f32, y: 150., sprite: block_sprite.clone() });
    }
    world.blocks.push(Block{ x: 100., y: 150. + 32., sprite: block_sprite.clone() });
    world.blocks.push(Block{ x: 100., y: 150. + 64., sprite: block_sprite.clone() });

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

        if keyboard::is_key_pressed(keyboard::W) { entity.move(North, dt, &world) }
        if keyboard::is_key_pressed(keyboard::D) { entity.move(East, dt, &world) }
        if keyboard::is_key_pressed(keyboard::S) { entity.move(South, dt, &world) }
        if keyboard::is_key_pressed(keyboard::A) { entity.move(West, dt, &world) }
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
        entity.draw(&mut window);
        world.draw(&mut window);
        window.draw(&fps_text);
        window.display()
    }
}
