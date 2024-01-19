use crate::map::coord::Coord;

#[derive(Debug, Clone)]
pub enum Action<'a> {
    Item(&'a str),
    Attack(Coord, usize),
    Talk(Coord),
    Use(Coord),
}
