mod pebble;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy rock :D".to_string(), 
                    resizable: false,
                    resolution: WindowResolution::new(512.0, 1024.0),
                    ..Default::default()
                }),
                ..Default::default()
            }
        ),
        pebble::PebblePlugin::default()
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}