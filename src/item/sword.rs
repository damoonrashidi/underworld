use std::{ops::SubAssign, time::Duration};

use crate::entity::Entity;

use super::Item;

#[derive(Debug)]
pub struct Sword {
    dmg: usize,
    cooldown: Duration,
}

impl Sword {
    #[must_use]
    pub fn new(dmg: usize, cooldown: Duration) -> Self {
        Self { dmg, cooldown }
    }
}

impl Item for Sword {
    fn on_use(&mut self) {
        println!("used sword");
        self.cooldown = Duration::from_millis(500);
    }
}

impl Entity for Sword {
    fn get_id(&self) -> String {
        String::from("sword")
    }

    fn on_tick(&mut self) {
        self.cooldown.sub_assign(Duration::from_millis(8));
    }
}
