extern crate native;
extern crate rsfml;
extern crate time;

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Color, Text};

use assets::Assets;
use components::{Mobile, Draw};
use sword::Sword;
use world::{World, North, East, South, West};

mod components;
mod assets;
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

    let assets = Assets::new();

    let mut world = World::new_from_file("resources/worlds/basic.txt", &assets);

    let sword = box Sword::new(0., 0., &assets);
    world.player.borrow_mut().set_active_item(sword);

    let mut last_time = time::precise_time_ns();
    let mut fps_last_time = last_time;
    let mut fps_count = 0u;
    let mut fps_text = Text::new_init("", assets.get_font(), 48).expect("error creating text");
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
        if keyboard::is_key_pressed(keyboard::W) { world.player.borrow_mut().move(North, dt, &world) }
        if keyboard::is_key_pressed(keyboard::D) { world.player.borrow_mut().move(East, dt, &world) }
        if keyboard::is_key_pressed(keyboard::S) { world.player.borrow_mut().move(South, dt, &world) }
        if keyboard::is_key_pressed(keyboard::A) { world.player.borrow_mut().move(West, dt, &world) }
        for event in window.events() {
            match event {
                event::KeyPressed{ code: keyboard::Escape, .. } |
                event::Closed => window.close(),
                event::KeyPressed{ code: keyboard::Space, .. } => {
                    world.player.borrow_mut().set_active_item_state(true);
                }
                event::KeyReleased{ code: keyboard::Space, .. } => {
                    world.player.borrow_mut().set_active_item_state(false);
                }
                event::KeyPressed{ code: _code, .. } => {
                    //println!("{}", _code);
                }
                _ => {}
            }
        }

        // Tick entities
        world.player.borrow_mut().tick(dt, &world);
        world.moblin.borrow_mut().tick(dt, &world);

        // Draw
        window.clear(&Color::new_RGB(50, 50, 50));
        world.player.borrow_mut().draw(&mut window);
        world.moblin.borrow_mut().draw(&mut window);
        world.draw(&mut window);
        window.draw(&fps_text);
        window.display()
    }
}
