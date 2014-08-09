use rsfml::graphics::{RenderWindow};
use world::{World, Direction};

pub trait Entity{}

pub trait Draw : Entity {
    fn draw(&mut self, w: &mut RenderWindow);
}

pub trait Mobile : Entity {
    fn move(&mut self, direction: Direction, dt: u64, world: &World);
}
