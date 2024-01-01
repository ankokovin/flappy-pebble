use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct PebblePlugin {}

impl Plugin for PebblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pebble)
            .add_systems(FixedUpdate, pebble_deccelerate)
            .add_systems(Update, (player_input, pebble_move));
    }
}

#[derive(Debug, Component)]
pub struct Pebble {
    velocity: f32
}

const PEBBLE_DEFAULT_VELOCITY: f32 = 400.0;

impl Default for Pebble {
    fn default() -> Self {
        Self { velocity: PEBBLE_DEFAULT_VELOCITY }
    }
}

fn spawn_pebble(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pebble.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                flip_x: true,
                custom_size: Some(Vec2 { x: 90.0, y: 90.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        Pebble::default()
    ));
}

const G_FORCE_ACCELERATION: f32 = -400.0;

fn pebble_deccelerate(time: Res<Time<Fixed>>, mut pebble: Query<&mut Pebble>) {
    let mut pebble = pebble.get_single_mut().expect("to get a pebble");
    pebble.velocity += G_FORCE_ACCELERATION * time.delta_seconds();
}

fn pebble_move(time: Res<Time>, mut transform_pebble: Query<&mut Transform, With<Pebble>>, pebble: Query<&Pebble>) {
    let mut transform = transform_pebble.get_single_mut().expect("to get a pebble transform");
    let pebble = pebble.get_single().expect("to get a pebble");
    transform.translation.y += pebble.velocity * time.delta_seconds();
}

fn player_input(
    input: Res<Input<KeyCode>>,
    mut pebble: Query<&mut Pebble>
) {
    if input.just_pressed(KeyCode::Space) {
        let mut pebble = pebble.get_single_mut().expect("to get a pebble");
        pebble.velocity = PEBBLE_DEFAULT_VELOCITY;
    }
}
