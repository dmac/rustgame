use std::io::BufferedReader;
use std::io::File;

use rsfml::graphics::{RenderWindow, Sprite, FloatRect};

use assets::Assets;
use components::{Bounded};
use util;

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
    MoblinStart,
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
    pub fn new(assets: &Assets) -> World {
        let wall_sprite = Sprite::new_with_texture(assets.get_texture("wall")).unwrap();
        World{
            tiles: Vec::new(),
            wall_sprite: wall_sprite,
        }
    }

    pub fn new_from_file(filepath: &str, assets: &'a Assets) -> World<'a> {
        let path = Path::new(filepath);
        let mut file = BufferedReader::new(File::open(&path));
        let mut world = World::new(assets);
        for (row, line) in file.lines().enumerate() {
            for (col, c) in line.unwrap().as_slice().chars().enumerate() {
                match c {
                    '-' | '|' => world.tiles.push(Tile{ row: row, col: col, kind: Wall }),
                    '@' => world.tiles.push(Tile{ row: row, col: col, kind: PlayerStart }),
                    'm' => world.tiles.push(Tile{ row: row, col: col, kind: MoblinStart }),
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

    pub fn collide_entity_with_tiles<T: Bounded>(&self, entity: &mut T, direction: Direction) {
        let (entity_x, entity_y, entity_width, entity_height) = entity.get_bounds();
        let entity_aabb = FloatRect::new(entity_x, entity_y, entity_width, entity_height);

        for &tile in self.tiles.iter() {
            let passable = match tile.kind {
                Wall => false,
                _ => true,
            };
            if passable { continue }
            let (tile_x, tile_y, tile_width, tile_height) = self.get_tile_bounds(tile);
            let tile_aabb = FloatRect::new(tile_x, tile_y, tile_width, tile_height);
            match util::collide_rects(&entity_aabb, &tile_aabb, direction) {
                Some((new_x, new_y)) => {
                    entity.set_x(new_x);
                    entity.set_y(new_y);
                }
                None => {}
            }
        }
    }
}
