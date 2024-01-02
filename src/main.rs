mod gamesize;
mod screen_entity;
mod state;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let height = 1024.0;
    let width = 512.0;
    let mut app = App::new();
    app.insert_resource(gamesize::GameSize::new(width, height))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy Pebble :D".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(width, height),
                    ..Default::default()
                }),
                ..Default::default()
            }),
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
