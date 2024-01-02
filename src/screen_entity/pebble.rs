use crate::consts::Consts;
use bevy::prelude::*;
use rand::Rng;

use crate::game_size::GameSize;
use crate::state::gamestate::GameState;

use super::moai::Moai;

#[derive(Debug, Default)]
pub struct PebblePlugin;

impl Plugin for PebblePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pebble>()
            .add_systems(
                OnEnter(GameState::Playing),
                (despawn_pebble, spawn_pebble).chain(),
            )
            .add_systems(OnEnter(GameState::MainMenu), despawn_pebble)
            .add_systems(
                FixedUpdate,
                (pebble_move, check_death_down, check_collisions)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                render_pebble.run_if(not(in_state(GameState::MainMenu))),
            )
            .add_systems(Update, player_input.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Pebble {
    velocity: f32,
    x: f32,
    y: f32,
}

impl Pebble {
    fn new(y: f32, velocity: f32) -> Pebble {
        Pebble {
            velocity,
            x: 0.0,
            y,
        }
    }
}

fn spawn_pebble(mut commands: Commands, asset_server: Res<AssetServer>, consts: Res<Consts>) {
    let mut rng = rand::thread_rng();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pebblesona.png"),
            sprite: Sprite {
                flip_x: true,
                custom_size: Some(Vec2 {
                    x: consts.pebble_width,
                    y: consts.pebble_height,
                }),
                ..Default::default()
            },
            ..Default::default()
        },
        Pebble::new(
            rng.gen_range(consts.pebble_start_y_range.clone()),
            consts.pebble_default_velocity,
        ),
        Name::new("Pebble"),
    ));
}

fn pebble_move(time: Res<Time<Fixed>>, mut pebble: Query<&mut Pebble>, consts: Res<Consts>) {
    let mut pebble = pebble.get_single_mut().expect("to get a pebble");
    pebble.y += pebble.velocity * time.delta_seconds()
        + consts.g_force_acceleration * time.delta_seconds() * time.delta_seconds() / 2.0;
    pebble.velocity += consts.g_force_acceleration * time.delta_seconds();
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
}

fn player_input(input: Res<Input<KeyCode>>, mut pebble: Query<&mut Pebble>, consts: Res<Consts>) {
    if input.just_pressed(KeyCode::Space) {
        let mut pebble = pebble.get_single_mut().expect("to get a pebble");
        pebble.velocity = consts.pebble_default_velocity;
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
    consts: Res<Consts>,
) {
    let pebble = query_pebble.get_single().expect("to get a pebble");
    for moai in query_moai.iter() {
        let already_passed =
            moai.x + consts.moai_width / 2.0 < pebble.x - consts.pebble_width / 2.0;
        let not_reached_yet =
            moai.x - consts.moai_width / 2.0 > pebble.x + consts.pebble_width / 2.0;
        if not_reached_yet || already_passed {
            continue;
        }

        let collided_down = moai.height > pebble.y - consts.pebble_height / 2.0;
        let up_moai_start_y = moai.height + consts.moai_vertical_distance;
        let collided_up = up_moai_start_y < pebble.y + consts.pebble_height / 2.0;

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
