use self::{tile::Tile, tiletype::TileType};

pub mod coord;
pub mod direction;
pub mod tile;
pub mod tiletype;

pub const TILE_SIZE: usize = 20;

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tiles: vec![],
            width: 0,
            height: 0,
        }
    }

    #[must_use]
    pub fn get_width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn get_height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn tile_is_in_viewport(tile: &Tile, origin: &coord::Coord) -> bool {
        tile.pos.0.abs_diff(origin.0) < 20 && tile.pos.1.abs_diff(origin.1) < 20
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

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

        let (width, height) = tiles.iter().fold((0, 0), |(max_width, max_height), tile| {
            let width = if tile.pos.0 > max_width {
                tile.pos.0
            } else {
                max_width
            };
            let height = if tile.pos.1 > max_height {
                tile.pos.1
            } else {
                max_height
            };

            (width, height)
        });

        Self {
            tiles,
            width,
            height,
        }
    }
}
