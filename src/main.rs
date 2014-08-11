extern crate native;
extern crate rsfml;
extern crate time;

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Texture, Sprite, Color, Font, Text};

use components::{Mobile, Draw};
use player::{Player};
use world::{World, PlayerStart, North, East, South, West};

mod components;
mod player;
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
    let wall_texture = Texture::new_from_file("resources/block.gif").expect("error loading texture");
    let player_sprite = Sprite::new_with_texture(&player_texture).expect("error creating sprite");
    let wall_sprite = Sprite::new_with_texture(&wall_texture).expect("error creating sprite");

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

    let mut last_time = time::precise_time_ns();
    let mut fps_last_time = last_time;
    let mut fps_count = 0u;
    let mut fps_text = Text::new_init("", &font, 48).expect("error creating text");
    while window.is_open() {
        fps_count += 1;
        let curr_time = time::precise_time_ns();
        let dt = curr_time - last_time;
        last_time = curr_time;
        if curr_time - fps_last_time > 1000000000 {
            fps_text.set_string(fps_count.to_string().as_slice());
            fps_last_time = curr_time;
            fps_count = 0;
        }

        if keyboard::is_key_pressed(keyboard::W) { player.move(North, dt, &world) }
        if keyboard::is_key_pressed(keyboard::D) { player.move(East, dt, &world) }
        if keyboard::is_key_pressed(keyboard::S) { player.move(South, dt, &world) }
        if keyboard::is_key_pressed(keyboard::A) { player.move(West, dt, &world) }
        for event in window.events() {
            match event {
                event::KeyPressed{ code: keyboard::Escape, .. } |
                event::Closed => window.close(),
                event::KeyPressed{ code: _code, .. } => {
                    // println!("{}", _code);
                }
                _ => {}
            }
        }

        window.clear(&Color::new_RGB(50, 50, 50));
        player.draw(&mut window);
        world.draw(&mut window);
        window.draw(&fps_text);
        window.display()
    }
}
