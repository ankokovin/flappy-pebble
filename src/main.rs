mod gamesize;
mod gamestate;
mod pebble;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use gamestate::GameState;

fn main() {
    let height = 1024.0;
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy rock :D".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(512.0, height),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            //max_y might become lower
            pebble::PebblePlugin::new(-height / 2.0, height / 2.0),
            ui::UiPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
