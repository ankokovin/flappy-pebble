mod consts;
mod gamesize;
mod screen_entity;
mod state;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;



#[cfg(target_family = "wasm")]
fn get_window(_window_name: &str, _width: f32, _height: f32) -> Window {
    Window {
        fit_canvas_to_parent: true,
        ..default()
    }
}

#[cfg(not(target_family = "wasm"))]
fn get_window(window_name: &str, width: f32, height: f32) -> Window {
    Window {
        title: window_name.to_string(),
        resizable: true,
        resolution: bevy::window::WindowResolution::new(width, height),
        ..Default::default()
    }
}

fn main() {
    let default_height = consts::WINDOW_HEIGHT;
    let default_width = consts::WINDOW_WIDTH;
    let window_name = consts::WINDOW_NAME;
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
                primary_window: Some(get_window(window_name, default_width, default_height)),
                ..Default::default()
            }),
        gamesize::GameSizeChangePlugin::new(default_width, default_height),
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
