use rsfml::graphics::{RenderWindow, Sprite};

pub enum TileKind {
    Wall,
}

pub struct Tile {
    pub row: uint,
    pub col: uint,
    pub kind: TileKind,
}

pub struct World<'a> {
    pub tiles: Vec<Tile>,
    wall_sprite: Sprite<'a>,
}

impl<'a> World<'a> {
    pub fn new(wall_sprite: Sprite) -> World {
        World{
            tiles: Vec::new(),
            wall_sprite: wall_sprite,
        }
    }

    pub fn get_tile_bounds(&self, tile: Tile) -> (f32, f32, f32, f32) {
        let bounds = self.wall_sprite.get_local_bounds();
        (tile.col as f32 * bounds.width, tile.row as f32 * bounds.height,
         bounds.width, bounds.height)
    }

    pub fn draw(&mut self, w: &mut RenderWindow) {
        for &tile in self.tiles.iter() {
            let (x, y, _, _) = self.get_tile_bounds(tile);
            match tile.kind {
                Wall => {
                    self.wall_sprite.set_position2f(x, y);
                    w.draw(&self.wall_sprite);
                }
            }
        }
    }
}
