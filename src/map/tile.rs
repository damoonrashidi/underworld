use super::{coord::Coord, tiletype::TileType};
use crate::entity::renderable::Renderable;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video};

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub enum TileState {
    Visible,
    Seen,
    Unseen,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub pos: Coord,
    pub kind: TileType,
    pub state: TileState,
}

impl Renderable for Tile {
    fn render(&self, ctx: &mut Canvas<video::Window>) -> Result<(), String> {
        let color = match self.kind {
            TileType::Empty => Color::RGB(0, 0, 0),
            TileType::Ground => Color::RGB(200, 200, 200),
            TileType::Grass => Color::RGB(0, 255, 0),
            TileType::Bush => Color::RGB(200, 255, 0),
            TileType::Tree => Color::RGB(40, 255, 40),
            TileType::Rock => Color::RGB(90, 90, 90),
            TileType::Wall => Color::RGB(60, 60, 60),
            TileType::Door => Color::RGB(8, 70, 40),
            TileType::Water => Color::RGB(50, 80, 200),
        };

        ctx.set_draw_color(color);

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        ctx.fill_rect(Rect::new(
            self.pos.0 as i32 * 40,
            self.pos.1 as i32 * 40,
            40,
            40,
        ))?;
        Ok(())
    }
}
