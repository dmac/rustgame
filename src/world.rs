use std::cell::RefCell;
use std::rc::Rc;
use std::io::BufferedReader;
use std::io::File;

use rsfml::graphics::{RenderWindow, Sprite, FloatRect};

use assets::Assets;
use components::{Bounded, Draw};
use player::Player;
use enemies::Enemy;
use util;

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[deriving(Eq, PartialEq, Show, Clone)]
enum TileKind {
    Empty,
    Wall,
    PlayerStart,
    EnemyStart,
}

#[deriving(Show, Clone)]
pub struct Tile {
    pub row: uint,
    pub col: uint,
    kind: TileKind,
}

pub struct World<'a> {
    pub player: RefCell<Player<'a>>,
    pub enemies: RefCell<Vec<RefCell<Enemy<'a>>>>,
    tiles: Vec<Rc<RefCell<Tile>>>,
    wall_sprite: Sprite<'a>,
}

impl<'a> World<'a> {
    pub fn new_from_file(filepath: &str, assets: &'a Assets) -> World<'a> {
        let wall_sprite = Sprite::new_with_texture(&assets.textures.wall).unwrap();
        let mut tiles = Vec::new();
        let player = RefCell::new(Player::new(0., 0., 200., assets));
        let mut enemies = Vec::new();
        let path = Path::new(filepath);
        let mut file = BufferedReader::new(File::open(&path));
        for (row, line) in file.lines().enumerate() {
            for (col, c) in line.unwrap().as_slice().chars().enumerate() {
                let tile = match c {
                    '-' | '|' => Some(Tile{ row: row, col: col, kind: Wall }),
                    '@' => {
                        let tile = Tile{ row: row, col: col, kind: PlayerStart };
                        let bounds = wall_sprite.get_local_bounds();
                        let (x, y) = (tile.col as f32 * bounds.width, tile.row as f32 * bounds.height);
                        player.borrow_mut().set_x(x);
                        player.borrow_mut().set_y(y);
                        Some(tile)
                    },
                    'm' => {
                        let tile = Tile{ row: row, col: col, kind: EnemyStart };
                        let bounds = wall_sprite.get_local_bounds();
                        let enemy = RefCell::new(
                            Enemy::new(tile.col as f32 * bounds.width, tile.row as f32 * bounds.height,
                                        assets));
                        enemies.push(enemy);
                        Some(tile)
                    },
                    _ => None
                };
                match tile {
                    Some(tile) => tiles.push(Rc::new(RefCell::new(tile))),
                    None => {}
                };
            }
        }
        World{
            player: player,
            enemies: RefCell::new(enemies),
            tiles: tiles,
            wall_sprite: wall_sprite,
        }
    }

    pub fn get_tile_bounds(&self, tile: &Tile) -> (f32, f32, f32, f32) {
        let bounds = self.wall_sprite.get_local_bounds();
        (tile.col as f32 * bounds.width, tile.row as f32 * bounds.height,
         bounds.width, bounds.height)
    }

    pub fn tile_at(&self, row: uint, col: uint) -> Rc<RefCell<Tile>> {
        let rc = self.tiles.iter().find(|tile| {
            let tile = tile.borrow();
            tile.row == row && tile.col == col
        }).or_else(|| None );
        match rc {
            Some(rc) => rc.clone(),
            None => Rc::new(RefCell::new(Tile{ row: row, col: col, kind: Empty }))
        }
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
            let dead = enemies.deref()[i].borrow().health <= 0;
            if dead {
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
        for tile in self.tiles.iter() {
            let tile = tile.borrow();
            let (x, y, _, _) = self.get_tile_bounds(tile.deref());
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

        for tile in self.tiles.iter() {
            let tile = tile.borrow();
            let passable = match tile.kind {
                Wall => false,
                _ => true,
            };
            if passable { continue }
            let (tile_x, tile_y, tile_width, tile_height) = self.get_tile_bounds(tile.deref());
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
