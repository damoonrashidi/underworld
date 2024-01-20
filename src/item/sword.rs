use super::Item;
use crate::{action::Action, entity::Entity, state::State};
use std::{cell::RefCell, rc::Rc, time::Duration};

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
    fn on_use(&mut self, state: Rc<RefCell<State>>) {
        if self.cooldown.is_zero() {
            println!("sword attacked!");
            self.cooldown = Duration::from_millis(500);
            State::dispatch(state.clone(), &Action::Item(self.get_id()));
        }
    }
}

impl Entity for Sword {
    fn on_tick(&mut self) {
        self.cooldown = self.cooldown.saturating_sub(Duration::from_millis(16));
    }

    fn get_id(&self) -> String {
        "sword".into()
    }

    fn on_action(&mut self, action: Action, state: Rc<RefCell<State>>) {
        match action {
            Action::Item(id) if id == *"sword" => self.on_use(state),
            action => {
                println!("{action:?}");
            }
        }
    }
}
