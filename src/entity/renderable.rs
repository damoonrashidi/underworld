use sdl2::{render::Canvas, video::Window};

use crate::map::coord::Coord;

pub trait Renderable {
    /// # Errors
    /// If the target cannot be rendered
    fn render(&self, displace: &Coord, ctx: &mut Canvas<Window>) -> Result<(), String>;
}
