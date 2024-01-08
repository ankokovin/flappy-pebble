mod consts;
mod game_size;
mod gamepad_util;
mod screen_entity;
mod state;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(target_family = "wasm")]
fn get_window(_window_name: String, _width: f32, _height: f32) -> Window {
    Window {
        fit_canvas_to_parent: true,
        canvas: if cfg!(not(debug_assertions)) {
            Some("#game-canvas".into())
        } else {
            None
        },
        ..default()
    }
}

#[cfg(not(target_family = "wasm"))]
fn get_window(window_name: String, width: f32, height: f32) -> Window {
    Window {
        title: window_name,
        resizable: true,
        resolution: bevy::window::WindowResolution::new(width, height),
        ..default()
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(get_window(
                consts::WINDOW_NAME.to_string(),
                consts::WINDOW_WIDTH,
                consts::WINDOW_HEIGHT,
            )),
            ..default()
        }),
        state::StatePlugin,
        screen_entity::GameEntityPlugin,
        ui::UiPlugin,
        game_size::GameSizeChangePlugin::new(0.0, 0.0),
    ))
    .add_systems(Startup, spawn_camera);

    if cfg!(feature = "egui") {
        app.add_plugins(WorldInspectorPlugin::default());
    }

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
