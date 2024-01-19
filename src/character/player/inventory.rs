use crate::{entity::Entity, item::Item, state::State};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub struct Inventory {
    items: Vec<Box<dyn Item>>,
    capacity: usize,
}

impl Inventory {
    pub fn add(&mut self, item: Box<dyn Item>) {
        if self.items.len() < self.capacity {
            self.items.push(item);
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Box<dyn Item>> {
        self.items.get_mut(index)
    }

    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, index: usize) -> Option<&Box<dyn Item>> {
        self.items.get(index)
    }
}

impl Entity for Inventory {
    fn get_id(&self) -> String {
        "inventory".into()
    }

    fn on_tick(&mut self) {
        for item in &mut self.items {
            item.on_tick();
        }
    }

    fn on_action(&mut self, _action: crate::action::Action, _state: Rc<RefCell<State>>) {}
}

impl Debug for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "items: {}", self.items.len())
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![],
            capacity: 10,
        }
    }
}
