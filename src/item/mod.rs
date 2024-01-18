pub mod sword;

pub trait Item {
    fn on_use(&mut self);
}
