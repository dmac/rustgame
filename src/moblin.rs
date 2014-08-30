use rsfml::graphics::{RenderWindow, Sprite};

use assets::Assets;
use components::{Entity, Draw, Mobile, Bounded};
use world::{World, Direction, North, East, South, West};

pub struct Moblin<'a> {
    x: f32,
    y: f32,
    speed: f32,
    pub health: i32,
    max_health: i32,
    sprite: Sprite<'a>,
}

impl<'a> Moblin<'a>{
    pub fn new(x: f32, y: f32, assets: &Assets) -> Moblin {
        let sprite = Sprite::new_with_texture(assets.get_texture("moblin")).unwrap();
        Moblin{
            x: x,
            y: y,
            speed: 50.,
            health: 100,
            max_health: 100,
            sprite: sprite
        }
    }

    pub fn tick(&mut self, dt: u64, world: &World) {
        self.move(South, dt, world);
    }

    pub fn damage(&mut self, amount: i32) {
        self.health -= amount;
        println!("Moblin Health: {}/{}", self.health, self.max_health);
    }
}

impl<'a> Entity for Moblin<'a> {}

impl<'a> Draw for Moblin<'a> {
    fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x, self.y);
        w.draw(&self.sprite);
    }
}

impl<'a> Bounded for Moblin<'a> {
    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let local_bounds = self.sprite.get_local_bounds();
        (self.x, self.y, local_bounds.width, local_bounds.height)
    }

    fn set_bounds(&mut self, x: f32, y: f32, _: f32, _: f32) {
        self.x = x;
        self.y = y;
    }
}


impl<'a> Mobile for Moblin<'a> {
    fn move(&mut self, direction: Direction, dt: u64, world: &World) {
        let distance = self.speed * dt as f32 / (1e9 as f32);
        match direction {
            North => self.y -= distance,
            East => self.x += distance,
            South => self.y += distance,
            West => self.x -= distance,
        }

        world.collide_entity_with_tiles(self, direction);
    }
}
