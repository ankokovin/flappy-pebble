use bevy::prelude::*;

use crate::gamesize::GameSize;
use crate::gamestate::GameState;
use crate::moai::{Moai, MOAI_VERTICAL_DISTANCE, MOAI_WIDTH};

#[derive(Debug, Default)]
pub struct PebblePlugin;

impl Plugin for PebblePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pebble>()
            .add_systems(
                OnEnter(GameState::Playing),
                (despawn_pebble, spawn_pebble).chain(),
            )
            .add_systems(
                FixedUpdate,
                (pebble_move, check_death_down, check_collisions)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, render_pebble)
            .add_systems(Update, player_input.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Component, Reflect)]
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
            y: 0.0,
        }
    }
}

const PEBBLE_WIDTH: f32 = 90.0;
const PEBBLE_HEIGHT: f32 = 52.0;

fn spawn_pebble(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pebblesona.png"),
            sprite: Sprite {
                flip_x: true,
                custom_size: Some(Vec2 {
                    x: PEBBLE_WIDTH,
                    y: PEBBLE_HEIGHT,
                }),
                ..Default::default()
            },
            ..Default::default()
        },
        Pebble::default(),
        Name::new("Pebble"),
    ));
}

const G_FORCE_ACCELERATION: f32 = -400.0;

fn pebble_move(time: Res<Time<Fixed>>, mut pebble: Query<&mut Pebble>) {
    let mut pebble = pebble.get_single_mut().expect("to get a pebble");
    pebble.y += pebble.velocity * time.delta_seconds()
        + G_FORCE_ACCELERATION * time.delta_seconds() * time.delta_seconds() / 2.0;
    pebble.velocity += G_FORCE_ACCELERATION * time.delta_seconds();
}

fn render_pebble(
    mut transform_pebble: Query<&mut Transform, With<Pebble>>,
    pebble: Query<&Pebble>,
) {
    let mut transform = transform_pebble
        .get_single_mut()
        .expect("to get a pebble transform");
    let pebble = pebble.get_single().expect("to get a pebble");
    transform.translation.x = pebble.x;
    transform.translation.y = pebble.y;
    //transform.rotate_local_z(1.0e-1);
}

fn player_input(input: Res<Input<KeyCode>>, mut pebble: Query<&mut Pebble>) {
    if input.just_pressed(KeyCode::Space) {
        let mut pebble = pebble.get_single_mut().expect("to get a pebble");
        pebble.velocity = PEBBLE_DEFAULT_VELOCITY;
    }
}

fn check_death_down(
    query_pebble: Query<&Pebble>,
    game_size: Res<GameSize>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let pebble = query_pebble.get_single().expect("to get a pebble");
    if pebble.y < game_size.min_y {
        game_state.set(GameState::GameOver)
    }
}

//to run this check in FixedUpdate, collisions are going to be checked manually
fn check_collisions(
    query_pebble: Query<&Pebble>,
    query_moai: Query<&Moai>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let pebble = query_pebble.get_single().expect("to get a pebble");
    for moai in query_moai.iter() {
        let already_passed = moai.x + MOAI_WIDTH / 2.0 < pebble.x - PEBBLE_WIDTH / 2.0;
        let not_reached_yet = moai.x - MOAI_WIDTH / 2.0 > pebble.x + PEBBLE_WIDTH / 2.0;
        if not_reached_yet || already_passed {
            continue;
        }

        let collided_down = moai.height > pebble.y - PEBBLE_HEIGHT / 2.0;
        let collided_up = moai.height + MOAI_VERTICAL_DISTANCE < pebble.y + PEBBLE_HEIGHT / 2.0;

        dbg!(collided_up, collided_down);
        if collided_down || collided_up {
            game_state.set(GameState::GameOver);
        }
    }
}

fn despawn_pebble(mut commands: Commands, query_pebble: Query<Entity, With<Pebble>>) {
    for pebble in query_pebble.iter() {
        commands.entity(pebble).despawn_recursive();
    }
}
