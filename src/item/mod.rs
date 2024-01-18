use crate::entity::Entity;

pub mod sword;

pub trait Item: Entity {
    fn on_use(&mut self);
}
