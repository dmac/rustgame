use rsfml::graphics::{RenderWindow, Sprite};

use components::{Entity, Draw, Mobile, Bounded};
use world::{World, Direction, North, East, South, West};

pub struct Player<'a> {
    x: f32,
    y: f32,
    speed: f32, // pixels per second
    sprite: Sprite<'a>,
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, speed: f32, sprite: Sprite<'a>) -> Player {
        Player{x: x, y: y, speed: speed, sprite: sprite}
    }
}

impl<'a> Entity for Player<'a> {}

impl<'a> Draw for Player<'a> {
    fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x, self.y);
        w.draw(&self.sprite);
    }
}

impl<'a> Bounded for Player<'a> {
    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let local_bounds = self.sprite.get_local_bounds();
        (self.x, self.y, local_bounds.width, local_bounds.height)
    }

    fn set_bounds(&mut self, x: f32, y: f32, _: f32, _: f32) {
        self.x = x;
        self.y = y;
    }
}

impl<'a> Mobile for Player<'a> {
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
