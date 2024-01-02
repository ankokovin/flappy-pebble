mod consts;
mod game_size;
mod screen_entity;
mod state;
mod ui;
mod gamepad_util;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(target_family = "wasm")]
fn get_window(_window_name: String, _width: f32, _height: f32) -> Window {
    Window {
        fit_canvas_to_parent: true,
        ..default()
    }
}

#[cfg(not(target_family = "wasm"))]
fn get_window(window_name: String, width: f32, height: f32) -> Window {
    Window {
        title: window_name,
        resizable: true,
        resolution: bevy::window::WindowResolution::new(width, height),
        ..Default::default()
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(consts::ConstsPlugin);

    let consts = app
        .world
        .get_resource::<consts::Consts>()
        .expect("to have Consts resource");

    let default_height = consts.window_height;
    let default_width = consts.window_width;
    let window_name = consts.window_name.clone();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(get_window(window_name, default_width, default_height)),
            ..Default::default()
        }),
        game_size::GameSizeChangePlugin::new(default_width, default_height),
        state::StatePlugin,
        screen_entity::GameEntityPlugin,
        ui::UiPlugin,
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
