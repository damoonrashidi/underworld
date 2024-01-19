use sdl2::{render::Canvas, video::Window};

pub trait Renderable {
    /// # Errors
    /// If the target cannot be rendered
    fn render(&self, ctx: &mut Canvas<Window>) -> Result<(), String>;
}
