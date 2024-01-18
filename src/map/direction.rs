#[derive(Debug, Default, PartialEq)]
pub enum Direction {
    North,
    East,
    #[default]
    South,
    West,
}
