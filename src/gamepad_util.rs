
use bevy::input::Input;
use bevy::prelude::{GamepadButton, GamepadButtonType, Res};

//without local coop we do not care which gamepad this input came from
pub fn gamepad_pressed(
    gamepad_input: Res<Input<GamepadButton>>,
    button: GamepadButtonType) -> bool {
    gamepad_input.get_just_pressed()
        .any(|press| press.button_type == button)
}

pub fn gamepad_just_pressed(button: GamepadButtonType) -> impl FnMut(Res<Input<GamepadButton>>) -> bool + Clone
{
    move |inputs: Res<Input<GamepadButton>>| gamepad_pressed(inputs, button)
}
