use self::{tile::Tile, tiletype::TileType};

pub mod coord;
pub mod direction;
pub mod tile;
pub mod tiletype;

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
}

impl Map {}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let tiles: Vec<Tile> = value
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, kind)| Tile {
                        pos: coord::Coord(row, col),
                        kind: TileType::from(kind),
                        state: tile::TileState::Unseen,
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();

        Self { tiles }
    }
}
