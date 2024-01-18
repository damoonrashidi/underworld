pub mod hittable;

pub trait Entity {
    fn get_id(&self) -> String;
    fn on_tick(&mut self);
}
