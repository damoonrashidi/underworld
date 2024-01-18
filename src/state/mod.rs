use std::fmt::Debug;

use crate::{character::player::Player, entity::Entity};

pub struct State<'a> {
    pub player: Player,
    pub entities: Vec<Box<&'a mut dyn Entity>>,
}

impl<'a> State<'a> {
    #[must_use]
    pub fn new(player: Player) -> Self {
        Self {
            player,
            entities: vec![],
        }
    }

    pub fn add_entity(&mut self, entity: Box<&'a mut dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity: &'a mut Box<dyn Entity>) {
        if let Some(index) = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, target)| target.get_id() == entity.get_id())
            .map(|(i, _)| i)
        {
            self.entities.swap_remove(index);
        }
    }
}

impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "player: {:?} entities: {}",
            self.player,
            self.entities.len()
        )
    }
}
