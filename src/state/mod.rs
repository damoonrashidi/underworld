use crate::{action::Action, character::player::Player, entity::Entity, map::Map};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub struct State {
    pub player: Player,
    pub entities: Vec<Rc<RefCell<dyn Entity>>>,
    pub map: Map,
}

impl State {
    #[must_use]
    pub fn new(player: Player) -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(State {
            player,
            entities: vec![],
            map: Map { tiles: vec![] },
        }))
    }

    pub fn add_entity(&mut self, entity: Rc<RefCell<dyn Entity>>) {
        self.entities.push(entity);
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = map;
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn dispatch(state: Rc<RefCell<State>>, action: &Action) {
        let borrow_state = state.borrow();
        let entities = borrow_state.entities.clone();
        drop(borrow_state);

        for entity in entities {
            if let Ok(mut ref_mut_entity) = entity.try_borrow_mut() {
                ref_mut_entity.on_action(action.clone(), state.clone());
            }
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "player: {:?}", self.player,)
    }
}
