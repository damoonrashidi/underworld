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
            self.cooldown = Duration::from_millis(500);
            State::dispatch(state.clone(), &Action::Item("sword"));
        } else {
            println!("sword still has {}ms cooldown", self.cooldown.as_millis());
        }
    }
}

impl Entity for Sword {
    fn on_tick(&mut self) {
        self.cooldown = self.cooldown.saturating_sub(Duration::from_millis(8));
    }

    fn get_id(&self) -> String {
        "sword".into()
    }

    fn on_action(&mut self, action: Action, state: Rc<RefCell<State>>) {
        println!("sword on use happens here");
        match action {
            Action::Item("sword") => self.on_use(state),
            action => {
                println!("{action:?}");
            }
        }
    }
}
