extern crate native;
extern crate rsfml;
extern crate time;

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Texture, Sprite, Color, Font, Text};

use components::{Mobile, Draw};
use moblin::{Moblin};
use player::{Player};
use sword::{Sword};
use world::{World, PlayerStart, MoblinStart, North, East, South, West};

mod components;
mod moblin;
mod player;
mod sword;
mod util;
mod world;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() -> () {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "SFML Example",
                                       Close,
                                       &ContextSettings::default())
        .expect("error creating window");
    window.set_framerate_limit(60);

    let player_texture = Texture::new_from_file("resources/link.gif").expect("error loading texture");
    let moblin_texture = Texture::new_from_file("resources/moblin.gif").expect("error loading texture");
    let wall_texture = Texture::new_from_file("resources/block.gif").expect("error loading texture");
    let sword_texture = Texture::new_from_file("resources/sword.gif").expect("error loading texture");
    let player_sprite = Sprite::new_with_texture(&player_texture).expect("error creating sprite");
    let moblin_sprite = Sprite::new_with_texture(&moblin_texture).expect("error creating sprite");
    let wall_sprite = Sprite::new_with_texture(&wall_texture).expect("error creating sprite");
    let sword_sprite = Sprite::new_with_texture(&sword_texture).expect("error creating sprite");

    let font = Font::new_from_file("resources/Inconsolata-Regular.ttf").expect("error loading font");

    let mut world = World::new_from_file("resources/worlds/basic.txt", wall_sprite);

    let (startx, starty) = match world.tiles.iter().find(|tile| tile.kind == PlayerStart) {
        Some(&tile) => {
            let (x, y, _, _) = world.get_tile_bounds(tile);
            (x, y)
        }
        None => (0., 0.)
    };
    let mut player = Player::new(startx, starty, 200., player_sprite);

    let sword = box Sword::new(0., 0., sword_sprite);
    player.set_active_item(sword);

    // TODO: world should have a list of entities (including player)
    let (mstartx, mstarty) = match world.tiles.iter().find(|tile| tile.kind == MoblinStart) {
        Some(&tile) => {
            let (x, y, _, _) = world.get_tile_bounds(tile);
            (x, y)
        }
        None => (0., 0.)
    };
    let mut moblin = Moblin::new(mstartx, mstarty, moblin_sprite);

    let mut last_time = time::precise_time_ns();
    let mut fps_last_time = last_time;
    let mut fps_count = 0u;
    let mut fps_text = Text::new_init("", &font, 48).expect("error creating text");
    while window.is_open() {
        // Count frames and dt
        fps_count += 1;
        let curr_time = time::precise_time_ns();
        let dt = curr_time - last_time;
        last_time = curr_time;
        if curr_time - fps_last_time > 1000000000 {
            fps_text.set_string(fps_count.to_string().as_slice());
            fps_last_time = curr_time;
            fps_count = 0;
        }

        // Handle input
        if keyboard::is_key_pressed(keyboard::W) { player.move(North, dt, &world) }
        if keyboard::is_key_pressed(keyboard::D) { player.move(East, dt, &world) }
        if keyboard::is_key_pressed(keyboard::S) { player.move(South, dt, &world) }
        if keyboard::is_key_pressed(keyboard::A) { player.move(West, dt, &world) }
        for event in window.events() {
            match event {
                event::KeyPressed{ code: keyboard::Escape, .. } |
                event::Closed => window.close(),
                event::KeyPressed{ code: keyboard::Space, .. } => {
                    player.set_active_item_state(true);
                }
                event::KeyReleased{ code: keyboard::Space, .. } => {
                    player.set_active_item_state(false);
                }
                event::KeyPressed{ code: _code, .. } => {
                    //println!("{}", _code);
                }
                _ => {}
            }
        }

        // Tick entities
        player.tick(dt, &world);
        moblin.tick(dt, &world);

        // Draw
        window.clear(&Color::new_RGB(50, 50, 50));
        player.draw(&mut window);
        moblin.draw(&mut window);
        world.draw(&mut window);
        window.draw(&fps_text);
        window.display()
    }
}
