use crate::state::State;
use std::{cell::RefCell, rc::Rc};

use crate::action::Action;

pub mod hittable;
pub mod renderable;

pub trait Entity {
    fn get_id(&self) -> String;
    fn on_tick(&mut self);
    fn on_action(&mut self, action: Action, state: Rc<RefCell<State>>);
}
