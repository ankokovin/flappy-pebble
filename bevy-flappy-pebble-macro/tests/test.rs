use bevy::prelude::*;

use bevy_flappy_pebble_macro::ChangeStateButton;

#[derive(Debug, Default, Clone, Copy, States, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

pub trait ChangeStateButton where Self: Component + Sized {
    fn name(&self) -> String;

    fn should_change_state_keyboard(&self, input: Res<Input<KeyCode>>) -> bool;

    fn should_change_state_keyboard(input: Res<Input<KeyCode>>) -> bool;
    fn target_state() -> GameState;
}

#[derive(ChangeStateButton, Component)]
#[keyboard(Escape, A)]
#[target_state(MainMenu)]
struct Button;

#[derive(ChangeStateButton, Component)]
#[keyboard(Return)]
#[target_state(Playing)]
struct AnotherButton;


#[derive(ChangeStateButton, Component)]
#[target_state(GameOver)]
struct FinalButton;


fn main() {}