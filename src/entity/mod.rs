pub mod hittable;

pub trait Entity {
    fn on_tick(&mut self);
}
