use crate::item::Item;
use std::fmt::Debug;

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

    pub fn get(&mut self, index: usize) -> Option<&mut Box<dyn Item>> {
        self.items.get_mut(index)
    }
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
