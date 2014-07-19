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
        // TODO: This inefficiently iterates through all walls (even empty ones).
        for (i, &wall) in world.walls.iter().enumerate() {
            if !wall { continue; }
            let (wall_x, wall_y) = world.get_xy_from_index(i);
            let (wall_width, wall_height) = world.get_wall_bounds();
            let wall_aabb = FloatRect::new(wall_x, wall_y, wall_width, wall_height);
            if FloatRect::intersects(&aabb, &wall_aabb, &FloatRect::new(0.,0.,0.,0.)) {
                let mut new_x = self.x as i32;
                let mut new_y = self.y as i32;
                match direction {
                    North => {
                        while new_y < wall_y as i32 + wall_height as i32 {
                            new_y += 1;
                        }
                    }
                    East => {
                        while new_x + self.sprite.get_local_bounds().width as i32 > wall_x as i32 {
                            new_x -= 1;
                        }
                    }
                    South => {
                        while new_y + self.sprite.get_local_bounds().height as i32 > wall_y as i32 {
                            new_y -= 1;
                        }
                    }
                    West => {
                        while new_x < wall_x as i32 + wall_width as i32 {
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
    walls: Vec<bool>,
    max_width: uint,
    max_height: uint,
    wall_sprite: Sprite<'a>,
}

impl<'a> World<'a> {
    fn new(max_width: uint, max_height: uint, wall_sprite: Sprite<'a>) -> World<'a> {
        World{
            walls: Vec::from_elem(max_width*max_height, false),
            max_width: max_width,
            max_height: max_height,
            wall_sprite: wall_sprite,
        }
    }

    fn get_xy_from_index(&self, i: uint) -> (f32, f32) {
        let (row, col) = (i / self.max_height, i % self.max_width);
        let sprite_bounds = self.wall_sprite.get_local_bounds();
        (col as f32 * sprite_bounds.width, row as f32 * sprite_bounds.height)
    }

    fn get_wall_bounds(&self) -> (f32, f32) {
        let bounds = self.wall_sprite.get_local_bounds();
        (bounds.width, bounds.height)
    }

    fn draw(&mut self, w: &mut RenderWindow) {
        for (i, &wall) in self.walls.iter().enumerate() {
            if !wall { continue; }
            let (x, y) = self.get_xy_from_index(i);
            self.wall_sprite.set_position2f(x, y);
            w.draw(&self.wall_sprite);
        }
    }
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() -> () {
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

    let mut world = World::new(100, 100, block_sprite);

    for i in range(0u, 10) {
        world.walls.insert(i, true);
        world.walls.insert(i + 295, true);
    }

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
                event::KeyPressed{ code: _code, .. } => {
                    // println!("{}", _code);
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
