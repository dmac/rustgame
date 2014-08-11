use rsfml::graphics::{RenderWindow, Sprite};

use components::{Entity, Draw, Mobile, Bounded, Item};
use world::{World, Direction, North, East, South, West};

pub struct Player<'a, T> {
    x: f32,
    y: f32,
    speed: f32, // pixels per second
    sprite: Sprite<'a>,
    active_item: Option<Box<T>>,
}

impl<'a, T: Item + Bounded> Player<'a, T> {
    pub fn new(x: f32, y: f32, speed: f32, sprite: Sprite<'a>) -> Player<T> {
        Player{x: x, y: y, speed: speed, sprite: sprite, active_item: None}
    }

    pub fn set_active_item(&mut self, item: Box<T>) {
        self.active_item = Some(item);
    }

    pub fn set_active_item_state(&mut self, state: bool) {
        match self.active_item {
            Some(ref mut item) => {
                if state {
                    item.activate();
                } else {
                    item.deactivate();
                }
            }
            None => {}
        }
    }

    pub fn tick(&mut self, dt: u64, world: &World) {
        match self.active_item {
            Some(ref mut item) => item.tick(dt, world),
            None => {}
        }
    }
}

impl<'a, T> Entity for Player<'a, T> {}

impl<'a, T: Bounded + Draw> Draw for Player<'a, T> {
    fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x, self.y);
        w.draw(&self.sprite);

        match self.active_item {
            Some(ref mut item) => {
                item.set_x(self.x + 15.);
                item.set_y(self.y + 30.);
                item.draw(w);
            }
            None => {}
        }
    }
}

impl<'a, T> Bounded for Player<'a, T> {
    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let local_bounds = self.sprite.get_local_bounds();
        (self.x, self.y, local_bounds.width, local_bounds.height)
    }

    fn set_bounds(&mut self, x: f32, y: f32, _: f32, _: f32) {
        self.x = x;
        self.y = y;
    }
}

impl<'a, T> Mobile for Player<'a, T> {
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
