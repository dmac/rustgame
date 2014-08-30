use rsfml::graphics::{RenderWindow};
use world::{World, Direction};

pub trait Entity{}

pub trait Draw : Entity {
    fn draw(&mut self, w: &mut RenderWindow);
}

pub trait Mobile : Entity {
    fn move(&mut self, direction: Direction, dt: u64, world: &World);
}

pub trait Bounded : Entity {
    fn get_bounds(&self) -> (f32, f32, f32, f32);
    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32);

    fn set_x(&mut self, x: f32) {
        let (_, y, width, height) = self.get_bounds();
        self.set_bounds(x, y, width, height);
    }

    fn set_y(&mut self, y: f32) {
        let (x, _, width, height) = self.get_bounds();
        self.set_bounds(x, y, width, height);
    }
}

pub trait Item : Entity + Bounded + Draw {
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn tick(&mut self, dt: u64, world: &World);
}
