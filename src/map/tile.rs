#[derive(Debug)]
pub enum Tile {
    Empty,
    Ground,
    Grass,
    Bush,
    Tree,
    Rock,
    Wall,
    Door,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::Empty,
            '.' => Self::Ground,
            '"' => Self::Grass,
            '*' => Self::Bush,
            'o' => Self::Rock,
            'T' => Self::Tree,
            '#' => Self::Wall,
            'A' => Self::Door,
            _ => unreachable!(),
        }
    }
}
