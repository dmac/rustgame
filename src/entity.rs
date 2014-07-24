use rsfml::graphics::{RenderWindow, Sprite, FloatRect};

use world::{World, Direction, North, East, South, West, Wall};

pub struct Entity<'a> {
    x: f32,
    y: f32,
    speed: f32, // pixels per second
    sprite: Sprite<'a>,
}

impl<'a> Entity<'a> {
    pub fn new(x: f32, y: f32, speed: f32, sprite: Sprite<'a>) -> Entity {
        Entity{x: x, y: y, speed: speed, sprite: sprite}
    }

    pub fn move(&mut self, direction: Direction, dt: u64, world: &World) {
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
        for &tile in world.tiles.iter() {
            let passable = match tile.kind {
                Wall => false,
                _ => true,
            };
            if passable { continue }
            let (tile_x, tile_y, tile_width, tile_height) = world.get_tile_bounds(tile);
            let tile_aabb = FloatRect::new(tile_x, tile_y, tile_width, tile_height);
            if FloatRect::intersects(&aabb, &tile_aabb, &FloatRect::new(0.,0.,0.,0.)) {
                let mut new_x = self.x as i32;
                let mut new_y = self.y as i32;
                match direction {
                    North => {
                        while new_y < tile_y as i32 + tile_height as i32 {
                            new_y += 1;
                        }
                    }
                    East => {
                        while new_x + self.sprite.get_local_bounds().width as i32 > tile_x as i32 {
                            new_x -= 1;
                        }
                    }
                    South => {
                        while new_y + self.sprite.get_local_bounds().height as i32 > tile_y as i32 {
                            new_y -= 1;
                        }
                    }
                    West => {
                        while new_x < tile_x as i32 + tile_width as i32 {
                            new_x += 1;
                        }
                    }
                }
                self.x = new_x as f32;
                self.y = new_y as f32;
            }
        }
    }

    pub fn draw(&mut self, w: &mut RenderWindow) {
        self.sprite.set_position2f(self.x, self.y);
        w.draw(&self.sprite);
    }
}
