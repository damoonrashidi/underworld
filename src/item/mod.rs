use std::{cell::RefCell, rc::Rc};

use crate::{entity::Entity, state::State};

pub mod sword;

pub trait Item: Entity {
    fn on_use(&mut self, state: Rc<RefCell<State>>);
}
