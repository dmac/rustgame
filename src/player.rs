use rsfml::graphics::{RenderWindow, Sprite};

use assets::Assets;
use components::{Entity, Draw, Mobile, Bounded, Item};
use world::{World, Direction, North, East, South, West};

pub struct Player<'a> {
    x: f32,
    y: f32,
    speed: f32, // pixels per second
    sprite: Sprite<'a>,
    active_item: Option<Box<Item>>,
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, speed: f32, assets: &Assets) -> Player {
        let sprite = Sprite::new_with_texture(&assets.textures.player).unwrap();
        Player{x: x, y: y, speed: speed, sprite: sprite, active_item: None}
    }

    pub fn set_active_item(&mut self, item: Box<Item>) {
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

impl<'a> Entity for Player<'a> {}

impl<'a> Draw for Player<'a> {
    fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x, self.y);
        w.draw(&self.sprite);

        match self.active_item {
            Some(ref mut item) => {
                item.set_x(self.x + 9.);
                item.set_y(self.y + 25.);
                item.draw(w);
            }
            None => {}
        }
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
