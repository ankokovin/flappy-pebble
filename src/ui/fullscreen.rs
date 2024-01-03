use crate::gamepad_util::gamepad_just_pressed;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::WindowMode;

pub struct FullScreenPlugin;

impl Plugin for FullScreenPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(not(target_family = "wasm")) {
            app.add_systems(
                Update,
                toggle_fullscreen.run_if(
                    input_just_pressed(KeyCode::F)
                        .or_else(gamepad_just_pressed(GamepadButtonType::North)),
                ),
            );
        }
    }
}

fn toggle_fullscreen(mut window: Query<&mut Window>) {
    let mut window = window.single_mut();

    let current_mode = window.mode;

    window.mode = match current_mode {
        WindowMode::Windowed => WindowMode::Fullscreen,
        _ => WindowMode::Windowed,
    }
}
