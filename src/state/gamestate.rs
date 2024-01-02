use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, States, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
    Exit,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::Exit), exit);
    }
}

fn exit(mut exit: EventWriter<bevy::app::AppExit>) {
    exit.send(bevy::app::AppExit);
}