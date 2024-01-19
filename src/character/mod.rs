use crate::map::direction::Direction;

pub mod mood;
pub mod player;

pub trait Character {
    /**
    Moves the character to a new position based on a given direction. `move` is a keyword though.
    */
    fn reposition(&mut self, direction: Direction);
}
