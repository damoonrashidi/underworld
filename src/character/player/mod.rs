use std::{cell::RefCell, rc::Rc};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
    character::player::inventory::Inventory,
    entity::{hittable::Hittable, renderable::Renderable, Entity},
    map::{coord::Coord, direction::Direction},
    state::State,
};

use super::Character;

pub mod inventory;

#[derive(Debug)]
pub struct Player {
    pub pos: Coord,
    pub dir: Direction,
    hp: usize,
    pub inventory: Inventory,
}

impl Player {
    #[must_use]
    pub fn new(position: Coord) -> Self {
        Self {
            pos: position,
            hp: 100,
            dir: Direction::South,
            inventory: Inventory::default(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.inventory.add(item);
    }
}

impl Entity for Player {
    fn get_id(&self) -> String {
        "player".into()
    }

    fn on_tick(&mut self) {}

    fn on_action(&mut self, _action: crate::action::Action, _state: Rc<RefCell<State>>) {
        todo!()
    }
}

impl Hittable for Player {
    fn on_hit(&mut self, dmg: usize) {
        self.hp = self.hp.saturating_sub(dmg);
    }
}

impl Renderable for Player {
    fn render(&self, _displace: &Coord, ctx: &mut Canvas<Window>) -> Result<(), String> {
        ctx.set_draw_color(Color::RGB(0, 120, 40));
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        ctx.fill_rect(Rect::new(
            self.pos.0 as i32 * 40,
            self.pos.1 as i32 * 40,
            40,
            40,
        ))
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl Character for Player {
    fn walk(&mut self, direction: Direction) {
        self.pos = match direction {
            Direction::North => self.pos.up(),
            Direction::East => self.pos.right(),
            Direction::South => self.pos.down(),
            Direction::West => self.pos.left(),
        };

        self.dir = direction;
    }
}
