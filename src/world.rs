use std::cell::RefCell;
use std::io::BufferedReader;
use std::io::File;

use rsfml::graphics::{RenderWindow, Sprite, FloatRect};

use assets::Assets;
use components::{Bounded, Draw};
use player::Player;
use moblin::Moblin;
use util;

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[deriving(Eq, PartialEq, Show)]
enum TileKind {
    Wall,
    PlayerStart,
    MoblinStart,
}

#[deriving(Show)]
struct Tile {
    row: uint,
    col: uint,
    kind: TileKind,
}

pub struct World<'a> {
    pub player: RefCell<Player<'a>>,
    pub enemies: RefCell<Vec<RefCell<Moblin<'a>>>>,
    tiles: Vec<Tile>,
    wall_sprite: Sprite<'a>,
}

impl<'a> World<'a> {
    pub fn new_from_file(filepath: &str, assets: &'a Assets) -> World<'a> {
        let wall_sprite = Sprite::new_with_texture(assets.get_texture("wall")).unwrap();
        let mut tiles = Vec::new();
        let player = RefCell::new(Player::new(0., 0., 200., assets));
        let moblin = RefCell::new(Moblin::new(0., 0., assets));
        let path = Path::new(filepath);
        let mut file = BufferedReader::new(File::open(&path));
        for (row, line) in file.lines().enumerate() {
            for (col, c) in line.unwrap().as_slice().chars().enumerate() {
                match c {
                    '-' | '|' => tiles.push(Tile{ row: row, col: col, kind: Wall }),
                    '@' => tiles.push(Tile{ row: row, col: col, kind: PlayerStart }),
                    'm' => tiles.push(Tile{ row: row, col: col, kind: MoblinStart }),
                    _ => {}
                }
            }
        }
        match tiles.iter().find(|tile| tile.kind == PlayerStart) {
            Some(tile) => {
                let bounds = wall_sprite.get_local_bounds();
                let (x, y) = (tile.col as f32 * bounds.width, tile.row as f32 * bounds.height);
                player.borrow_mut().set_x(x);
                player.borrow_mut().set_y(y);
            }
            None => {}
        }
        match tiles.iter().find(|tile| tile.kind == MoblinStart) {
            Some(tile) => {
                let bounds = wall_sprite.get_local_bounds();
                let (x, y) = (tile.col as f32 * bounds.width, tile.row as f32 * bounds.height);
                moblin.borrow_mut().set_x(x);
                moblin.borrow_mut().set_y(y);
            }
            None => {}
        }
        World{
            player: player,
            enemies: RefCell::new(vec![moblin]),
            tiles: tiles,
            wall_sprite: wall_sprite,
        }
    }

    fn get_tile_bounds(&self, tile: Tile) -> (f32, f32, f32, f32) {
        let bounds = self.wall_sprite.get_local_bounds();
        (tile.col as f32 * bounds.width, tile.row as f32 * bounds.height,
         bounds.width, bounds.height)
    }

    pub fn tick(&self, dt: u64) {
        self.player.borrow_mut().tick(dt, self);
        for enemy in self.enemies.borrow().iter() {
            enemy.borrow_mut().tick(dt, self);
        }

        // Sweep and clear dead enemies
        let mut i = 0u;
        while i < self.enemies.borrow().len() {
            let mut enemies = self.enemies.borrow_mut();
            if enemies.deref()[i].borrow().health <= 0 {
                enemies.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn draw(&mut self, w: &mut RenderWindow) {
        self.player.borrow_mut().draw(w);
        for enemy in self.enemies.borrow().iter() {
            enemy.borrow_mut().draw(w);
        }
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
