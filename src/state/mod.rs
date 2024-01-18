use std::fmt::Debug;

use crate::character::player::Player;

pub struct State {
    pub player: Player,
}

impl State {
    #[must_use]
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "player: {:?}", self.player,)
    }
}
