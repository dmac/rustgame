use rsfml::graphics::{Sprite, RenderWindow};

use components::{Entity, Bounded, Draw, Item};
use world::{World};

pub struct Sword<'a> {
    x: f32,
    y: f32,
    active: bool,
    sprite: Sprite<'a>,
}

impl<'a> Sword<'a> {
    pub fn new(x: f32, y: f32, sprite: Sprite<'a>) -> Sword {
        Sword{x: x, y: y, sprite: sprite, active: false}
    }
}

impl<'a> Entity for Sword<'a> {}

impl<'a> Draw for Sword<'a> {
    fn draw(&mut self, w: &mut RenderWindow) {
        if self.active {
            self.sprite.set_position2f(self.x, self.y);
            w.draw(&self.sprite);
        }
    }
}

impl<'a> Bounded for Sword<'a> {
    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let local_bounds = self.sprite.get_local_bounds();
        (self.x, self.y, local_bounds.width, local_bounds.height)
    }

    fn set_bounds(&mut self, x: f32, y: f32, _: f32, _: f32) {
        self.x = x;
        self.y = y;
    }
}

impl<'a> Item for Sword<'a> {
    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn tick(&mut self, dt: u64, world: &World) {
        // TODO: check for damage
    }
}
