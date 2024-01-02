mod consts;
mod gamesize;
mod screen_entity;
mod state;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let height = consts::WINDOW_HEIGHT;
    let width = consts::WINDOW_WIDTH;
    let window_name = consts::WINDOW_NAME;
    let mut app = App::new();
    app.insert_resource(gamesize::GameSize::new(width, height))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: window_name.to_string(),
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
