use std::io::BufferedReader;
use std::io::File;

use rsfml::graphics::{RenderWindow, Sprite};

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[deriving(Eq, PartialEq, Show)]
pub enum TileKind {
    Wall,
    PlayerStart,
}

#[deriving(Show)]
struct Tile {
    row: uint,
    col: uint,
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

    pub fn new_from_file(filepath: &str, wall_sprite: Sprite<'a>) -> World<'a> {
        let path = Path::new(filepath);
        let mut file = BufferedReader::new(File::open(&path));
        let mut world = World::new(wall_sprite);
        for (row, line) in file.lines().enumerate() {
            for (col, c) in line.unwrap().as_slice().chars().enumerate() {
                match c {
                    '-' | '|' => world.tiles.push(Tile{ row: row, col: col, kind: Wall }),
                    '@' => world.tiles.push(Tile{ row: row, col: col, kind: PlayerStart }),
                    _ => {}
                }
            }
        }
        world
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
                _ => {}
            }
        }
    }
}
