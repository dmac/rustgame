use rsfml::graphics::{Sprite, RenderWindow, FloatRect};

use assets::Assets;
use components::{Entity, Bounded, Draw, Item};
use world::{World};

pub struct Sword<'a> {
    x: f32,
    y: f32,
    active: bool,
    damage: i32,
    sprite: Sprite<'a>,
}

impl<'a> Sword<'a> {
    pub fn new(x: f32, y: f32, assets: &Assets) -> Sword {
        let sprite = Sprite::new_with_texture(assets.get_texture("sword")).unwrap();
        Sword{
            x: x,
            y: y,
            damage: 10,
            sprite: sprite,
            active: false
        }
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

    fn tick(&mut self, _dt: u64, world: &World) {
        if !self.active {
            return;
        }
        for (i, enemy) in world.enemies.borrow().iter().enumerate() {
            let (x, y, w, h) = self.get_bounds();
            let srect = FloatRect::new(x, y, w, h);
            let (x, y, w, h) = enemy.borrow().get_bounds();
            let mrect = FloatRect::new(x, y, w, h);
            let intersects = FloatRect::intersects(&srect, &mrect, &FloatRect::new(0.,0.,0.,0.));
            if !intersects {
                continue;
            }
            enemy.borrow_mut().damage(self.damage);
        }
    }
}
