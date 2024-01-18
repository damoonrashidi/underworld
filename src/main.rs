extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::Duration;
use underworld::{
    character::{player::Player, Character},
    entity::Entity,
    item::sword::Sword,
    map::{coord, direction::Direction},
    render::Renderable,
    state::State,
};

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video_subsystem = ctx.video()?;
    let window = video_subsystem
        .window("Underworld", 800, 600)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let player = Player::new(coord::Coord(0, 0));
    let sword = Sword::new(5, Duration::from_millis(200));

    let mut state = State::new(player);
    state.player.add_item(Box::new(sword));

    println!("{state:?}");

    let mut event_pump = ctx.event_pump()?;

    'game_loop: loop {
        state.player.inventory.on_tick();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => state.player.use_item(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    state.player.reposition(Direction::West);
                    state.player.dir = Direction::West;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    state.player.reposition(Direction::North);
                    state.player.dir = Direction::North;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    state.player.reposition(Direction::East);
                    state.player.dir = Direction::East;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    state.player.reposition(Direction::South);
                    state.player.dir = Direction::South;
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game_loop;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 120, 0));
        state.player.render(&mut canvas)?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

pub fn run() {}
