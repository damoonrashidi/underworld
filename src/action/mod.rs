use crate::map::coord::Coord;

#[derive(Debug, Clone)]
pub enum Action {
    Item(String),
    Attack(Coord, usize),
    Talk(Coord),
    Use(Coord),
}
