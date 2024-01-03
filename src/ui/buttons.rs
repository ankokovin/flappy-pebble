use bevy::prelude::*;
pub use bevy_flappy_pebble_macro::*;
use crate::state::gamestate::GameState;

pub const DEFAULT_BUTTON_COLOR: Color = Color::GRAY;

pub trait ChangeStateButton where Self: Component + Sized {
    fn name(&self) -> String;

    fn should_change_state_keyboard(input: Res<Input<KeyCode>>) -> bool;

    fn target_state() -> GameState;

    fn button_pressed_system(interaction_query: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
                             mut next_state: ResMut<NextState<GameState>>) {
        for interaction in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(Self::target_state());
            }
        }
    }

    fn keyboard_pressed_system(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
        if Self::should_change_state_keyboard(input) {
            next_state.set(Self::target_state())
        }
    }
}

pub fn change_state_button<T>(
    button_bundle: ButtonBundle,
    component: T
) -> impl Bundle
    where T: ChangeStateButton + Bundle  {
    let name = component.name();

    (button_bundle, component, Name::new(name))
}

#[derive(Debug, Resource)]
pub struct SelectedButton;