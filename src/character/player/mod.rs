use crate::{
    character::player::inventory::Inventory,
    entity::{hittable::Hittable, Entity},
    item::Item,
    map::{coord::Coord, direction::Direction},
};

use super::Character;

pub mod inventory;

#[derive(Debug)]
pub struct Player {
    pub pos: Coord,
    hp: usize,
    pub inventory: Inventory,
}

impl Player {
    #[must_use]
    pub fn new(position: Coord) -> Self {
        Self {
            pos: position,
            hp: 100,
            inventory: Inventory::default(),
        }
    }

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        self.inventory.add(item);
    }

    pub fn use_item(&mut self, index: usize) {
        let Some(item) = self.inventory.get(index) else {
            println!("No item equipped at {index}");
            return;
        };

        item.on_use();
    }
}

impl Entity for Player {
    fn get_id(&self) -> String {
        String::from("player")
    }

    fn on_tick(&mut self) {}
}

impl Hittable for Player {
    fn on_hit(&mut self, dmg: usize) {
        self.hp = self.hp.saturating_sub(dmg);
    }
}

impl Character for Player {
    fn reposition(&mut self, direction: Direction) {
        self.pos = match direction {
            Direction::North => self.pos.up(),
            Direction::East => self.pos.right(),
            Direction::South => self.pos.down(),
            Direction::West => self.pos.left(),
        };
    }
}
