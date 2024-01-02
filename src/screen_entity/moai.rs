use bevy::prelude::*;

use crate::{gamesize::GameSize, state::gamescore::GameScore, state::gamestate::GameState};

pub struct MoaiPlugin;

impl Plugin for MoaiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Moai>()
            .add_systems(Startup, load_texture)
            .add_systems(
                OnEnter(GameState::Playing),
                (despawn_all_moai, spawn_init_moai).chain(),
            )
            .add_systems(Update, render_moai)
            .add_systems(
                Update,
                (despawn_moai_outside_screen).run_if(in_state(GameState::Playing)),
            )
            .add_systems(FixedUpdate, move_moai.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Moai {
    pub x: f32,
    pub height: f32,
}

pub const MOAI_WIDTH: f32 = 100.0;
const MOAI_HEIGHT: f32 = 1345.0;

#[derive(Resource)]
struct MoaiTexture {
    //TODO: separate head and body sections for better modularity
    handle: Handle<Image>,
}

impl MoaiTexture {
    fn new(handle: Handle<Image>) -> MoaiTexture {
        MoaiTexture { handle }
    }
}

fn load_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    let moai_texture = asset_server.load("moai.png");
    commands.insert_resource(MoaiTexture::new(moai_texture));
}

pub const MOAI_VERTICAL_DISTANCE: f32 = 300.0;

fn spawn_moai(mut commands: Commands, moai_texture: Res<MoaiTexture>, x: f32) {
    commands
        .spawn((
            SpatialBundle::default(),
            Moai { x, height: -100.0 },
            Name::new("Moai"),
        ))
        .with_children(|parent| {
            //down
            parent.spawn(SpriteBundle {
                texture: moai_texture.handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: MOAI_WIDTH,
                        y: MOAI_HEIGHT,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            });

            //up
            parent.spawn(SpriteBundle {
                texture: moai_texture.handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: MOAI_WIDTH,
                        y: MOAI_HEIGHT,
                    }),
                    flip_x: true,
                    flip_y: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3 {
                    x: 0.0,
                    y: MOAI_HEIGHT + MOAI_VERTICAL_DISTANCE,
                    z: 0.0,
                }),
                ..Default::default()
            });
        });
}

fn spawn_init_moai(commands: Commands, game_size: Res<GameSize>, moai_texture: Res<MoaiTexture>) {
    let x = game_size.max_x * 3.0;
    spawn_moai(commands, moai_texture, x)
}

fn despawn_all_moai(mut commands: Commands, query_all_moai: Query<Entity, With<Moai>>) {
    for moai in query_all_moai.iter() {
        commands.entity(moai).despawn_recursive();
    }
}

fn render_moai(mut query_all_moai: Query<(&Moai, &mut Transform)>) {
    for (moai, mut transform) in query_all_moai.iter_mut() {
        transform.translation.x = moai.x;
        transform.translation.y = moai.height - MOAI_HEIGHT / 2.0;
    }
}

const MOAI_MOVE_SPEED: f32 = 200.0;

const MOAI_HORIZONTAL_DISTANCE: f32 = 400.0;

fn move_moai(
    time: Res<Time<Fixed>>,
    mut query_all_moai: Query<&mut Moai>,
    commands: Commands,
    game_size: Res<GameSize>,
    moai_texture: Res<MoaiTexture>,
    mut game_score: ResMut<GameScore>,
) {
    let mut max_x = f32::MIN;
    for mut moai in query_all_moai.iter_mut() {
        let before = moai.x;
        let after = before - MOAI_MOVE_SPEED * time.delta_seconds();

        if before >= 0.0 && after <= 0.0 {
            game_score.inc_score();
        }

        moai.x = after;
        if moai.x > max_x {
            max_x = moai.x;
        }
    }

    if game_size.max_x + MOAI_WIDTH - max_x >= MOAI_HORIZONTAL_DISTANCE {
        let x = game_size.max_x;
        spawn_moai(commands, moai_texture, x);
    }
}

fn despawn_moai_outside_screen(
    mut commands: Commands,
    game_size: Res<GameSize>,
    query_all_moai: Query<(Entity, &Moai)>,
) {
    for (entity, moai) in query_all_moai.iter() {
        if moai.x < game_size.min_x - MOAI_WIDTH {
            commands.entity(entity).despawn_recursive();
        }
    }
}
