extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{cell::RefCell, rc::Rc, time::Duration};
use underworld::{
    action::Action,
    character::{player::Player, Character},
    entity::{renderable::Renderable, Entity},
    item::sword::Sword,
    map::{coord, direction::Direction, Map},
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

    let map = Map::from(include_str!("../maps/home.txt"));

    let mut player = Player::new(coord::Coord(0, 0));
    let sword = Sword::new(5, Duration::from_millis(200));
    player.add_item(sword.get_id());

    let state = State::new(player);
    let mut prep = state.borrow_mut();
    prep.add_entity(Rc::new(RefCell::new(sword)));
    prep.set_map(map);
    drop(prep);

    let mut event_pump = ctx.event_pump()?;

    'game_loop: loop {
        let borrow_state = state.borrow();
        let entities = borrow_state.entities.clone();
        drop(borrow_state);
        for entity in entities {
            let mut borrowed_entity = entity.borrow_mut();
            borrowed_entity.on_tick();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    State::dispatch(state.clone(), &Action::Item("sword".into()));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let mut state = state.borrow_mut();
                    state.player.walk(Direction::West);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let mut state = state.borrow_mut();
                    state.player.walk(Direction::North);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let mut state = state.borrow_mut();
                    state.player.walk(Direction::East);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let mut state = state.borrow_mut();
                    state.player.walk(Direction::South);
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

        for tile in state.borrow().map.clone().tiles {
            tile.render(&mut canvas)?;
        }
        canvas.set_draw_color(Color::RGB(0, 120, 0));
        state.borrow_mut().player.render(&mut canvas)?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

pub fn run() {}
