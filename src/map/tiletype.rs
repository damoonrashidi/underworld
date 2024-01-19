#[derive(Debug)]
pub enum TileType {
    Empty,
    Ground,
    Grass,
    Bush,
    Tree,
    Rock,
    Wall,
    Door,
    Water,
}

impl From<char> for TileType {
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
            '~' => Self::Water,
            _ => unreachable!(),
        }
    }
}
