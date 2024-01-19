use self::tiletype::TileType;

pub mod coord;
pub mod direction;
pub mod tile;
pub mod tiletype;

#[derive(Debug)]
pub struct Map<'a> {
    tiles: Vec<Vec<TileType>>,
    visible_tiles: &'a mut [TileType],
    seen_tiles: &'a mut [TileType],
}

impl<'a> Map<'a> {}

impl<'a> From<&str> for Map<'a> {
    fn from(value: &str) -> Self {
        let tiles: Vec<Vec<TileType>> = value
            .trim()
            .lines()
            .map(|line| line.chars().map(TileType::from).collect())
            .collect();

        Self {
            tiles,
            visible_tiles: &mut [],
            seen_tiles: &mut [],
        }
    }
}
