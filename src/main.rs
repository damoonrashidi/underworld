extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::time::Duration;
use underworld::{
    character::{player::Player, Character},
    entity::Entity,
    item::sword::Sword,
    map::{coord, direction::Direction},
    state::State,
};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Underworld", 800, 600)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut player = Player::new(coord::Coord(0, 0));
    let sword = Sword::new(5, Duration::from_millis(200));

    player.add_item(Box::new(sword));
    let mut state = State::new(player);

    println!("{state:?}");

    let mut event_pump = sdl_context.event_pump()?;

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
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    state.player.reposition(Direction::North);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    state.player.reposition(Direction::East);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    state.player.reposition(Direction::South);
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
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let _ = canvas.fill_rect(Rect::new(
            state.player.pos.0 as i32 * 40,
            state.player.pos.1 as i32 * 40,
            40,
            40,
        ));

        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

pub fn run() {}
