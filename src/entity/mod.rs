pub mod hittable;
pub mod renderable;

pub trait Entity {
    fn on_tick(&mut self);
}
