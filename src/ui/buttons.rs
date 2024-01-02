use bevy::prelude::*;
pub use bevy_flappy_pebble_macro::*;
use crate::state::gamestate::GameState;

pub const DEFAULT_BUTTON_COLOR: Color = Color::GRAY;

pub trait ChangeStateButton where Self: Component + Sized {
    fn name(&self) -> String;

    fn should_change_state_keyboard(&self, input: Res<Input<KeyCode>>) -> bool;

    fn target_state() -> GameState;

    fn interaction_system(interaction_query: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
                          next_state: ResMut<NextState<GameState>>);
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