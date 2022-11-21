use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod events;
mod game;

pub use events::*;
pub use game::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    DrawDefence,
    TestDefence,
    Win,
    Lose,
}
