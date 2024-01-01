use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, States, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}