use crate::map::direction::Direction;

pub mod mood;
pub mod player;

pub trait Character {
    fn reposition(&mut self, direction: Direction);
}
