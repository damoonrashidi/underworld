use self::tile::Tile;

pub mod coord;
pub mod direction;
pub mod tile;

#[derive(Debug)]
pub struct Map<'a> {
    tiles: Vec<Vec<Tile>>,
    visible_tiles: &'a mut [Tile],
    seen_tiles: &'a mut [Tile],
}

impl<'a> Map<'a> {}

impl<'a> From<&str> for Map<'a> {
    fn from(value: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = value
            .trim()
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        Self {
            tiles,
            visible_tiles: &mut [],
            seen_tiles: &mut [],
        }
    }
}
