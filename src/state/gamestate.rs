use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, States, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}
