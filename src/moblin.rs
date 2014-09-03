use std::rand::{task_rng, Rng};
use std::cell::RefCell;
use std::rc::Rc;
use rsfml::graphics::{RenderWindow, Sprite};

use assets::Assets;
use components::{Entity, Draw, Mobile, Bounded};
use world::{World, Tile, Direction, North, East, South, West};

pub struct Moblin<'a> {
    x: f32,
    y: f32,
    speed: f32,
    pub health: i32,
    max_health: i32,
    goal: Goal,
    sprite: Sprite<'a>,
}

struct Goal {
    tile: Option<Rc<RefCell<Tile>>>,
}

impl ::std::fmt::Show for Goal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::FormatError> {
        let mut tile = None;
        if self.tile.is_some() {
            tile = Some(self.tile.as_ref().unwrap().borrow());
        }
        f.write(format!("Goal {{ tile: {} }}", tile).as_bytes())
    }
}

impl<'a> Moblin<'a>{
    pub fn new(x: f32, y: f32, assets: &Assets) -> Moblin {
        let sprite = Sprite::new_with_texture(&assets.textures.moblin).unwrap();
        Moblin{
            x: x,
            y: y,
            speed: 50.,
            health: 100,
            max_health: 100,
            goal: Goal{ tile: None },
            sprite: sprite
        }
    }

    fn random_goal(&self, world: &World) -> Goal {
        let tile = self.goal.tile.as_ref().unwrap().borrow();
        let mut rng = task_rng();
        let row = (tile.row as int + rng.gen_range(-5, 5)) as uint;
        let col = (tile.col as int + rng.gen_range(-5, 5)) as uint;
        Goal{ tile: Some(world.tile_at(row, col)) }
    }

    fn direction_to_goal(&self, world: &World) -> Option<Direction> {
        match self.goal.tile {
            None => None,
            Some(ref tile) => {
                let tile = tile.borrow();
                let (tile_x, tile_y, _, _) = world.get_tile_bounds(tile.deref());
                let xdist = (tile_x - self.x) as int;
                let ydist = (tile_y - self.y) as int;
                if xdist > 0 { Some(East) }
                else if xdist < 0 { Some(West) }
                else if ydist > 0 { Some(South) }
                else if ydist < 0 { Some(North) }
                else { None }
            }
        }
    }

    pub fn tick(&mut self, dt: u64, world: &World) {
        if self.goal.tile.is_none() {
            self.goal.tile = Some(world.tile_at(7, 18));
        }

        match self.direction_to_goal(world) {
            Some(direction) => self.move(direction, dt, world),
            None => self.goal = self.random_goal(world)
        };
    }

    pub fn damage(&mut self, amount: i32) {
        self.health -= amount;
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
