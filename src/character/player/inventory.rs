use std::fmt::Debug;

pub struct Inventory {
    items: Vec<String>,
    capacity: usize,
}

impl Inventory {
    pub fn add(&mut self, item: String) {
        if self.items.len() < self.capacity {
            self.items.push(item);
        }
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
