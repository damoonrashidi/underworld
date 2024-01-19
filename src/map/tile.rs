use super::{coord::Coord, tiletype::TileType};
use crate::entity::renderable::Renderable;
use sdl2::{rect::Rect, render::Canvas, video};

#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum TileState {
    Visible,
    Seen,
    Unseen,
}

#[derive(Debug)]
pub struct Tile {
    pub pos: Coord,
    pub kind: TileType,
    pub state: TileState,
}

impl Renderable for Tile {
    fn render(&self, ctx: &mut Canvas<video::Window>) -> Result<(), String> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        ctx.draw_rect(Rect::new(
            self.pos.0 as i32 * 40,
            self.pos.1 as i32 * 40,
            40,
            40,
        ))?;
        Ok(())
    }
}
