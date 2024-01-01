use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::gamesize::GameSize;

#[derive(Debug)]
pub struct PebblePlugin {
   game_size: GameSize,
}

impl Plugin for PebblePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.game_size)
            .add_systems(OnEnter(GameState::Playing), spawn_pebble)
            .add_systems(FixedUpdate, (pebble_move, check_death).run_if(in_state(GameState::Playing)))
            .add_systems(Update, (player_input, render_pebble).run_if(in_state(GameState::Playing)));
    }
}

impl PebblePlugin {
    pub fn new(min_y: f32, max_y: f32) -> PebblePlugin {
        PebblePlugin {
            game_size: GameSize {
                min_y, 
                max_y
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct Pebble {
    velocity: f32,
    x: f32,
    y: f32,
}

const PEBBLE_DEFAULT_VELOCITY: f32 = 400.0;

impl Default for Pebble {
    fn default() -> Self {
        Self { 
            velocity: PEBBLE_DEFAULT_VELOCITY, 
            x: 0.0,
            y: 0.0 
        }
    }
}

fn spawn_pebble(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pebble.png"),
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

fn pebble_move(time: Res<Time<Fixed>>, mut pebble: Query<&mut Pebble>) {
    let mut pebble = pebble.get_single_mut().expect("to get a pebble");
    pebble.y += pebble.velocity * time.delta_seconds() + G_FORCE_ACCELERATION * time.delta_seconds() * time.delta_seconds() / 2.0;
    pebble.velocity += G_FORCE_ACCELERATION * time.delta_seconds();
}

fn render_pebble(mut transform_pebble: Query<&mut Transform, With<Pebble>>, pebble: Query<&Pebble>) {
    let mut transform = transform_pebble.get_single_mut().expect("to get a pebble transform");
    let pebble = pebble.get_single().expect("to get a pebble");
    transform.translation.x = pebble.x;
    transform.translation.y = pebble.y;
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

fn check_death(
    query_pebble: Query<&Pebble>,
    game_size: Res<GameSize>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let pebble = query_pebble.get_single().expect("to get a pebble");
    if pebble.y < game_size.min_y {
        game_state.set(GameState::GameOver)
    }
}