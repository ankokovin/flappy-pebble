use bevy::prelude::*;
use rand::Rng;

use crate::{
    consts, game_size::GameSize, state::gamescore::GameScore, state::gamestate::GameState,
};

pub struct MoaiPlugin;

impl Plugin for MoaiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Moai>()
            .add_systems(Startup, load_texture)
            .add_systems(
                OnTransition {
                    from: GameState::MainMenu,
                    to: GameState::Playing,
                },
                (despawn_all_moai, spawn_init_moai).chain(),
            )
            .add_systems(
                OnTransition {
                    from: GameState::GameOver,
                    to: GameState::Playing,
                },
                (despawn_all_moai, spawn_init_moai).chain(),
            )
            .add_systems(OnEnter(GameState::MainMenu), despawn_all_moai)
            .add_systems(Update, render_moai.run_if(in_state(GameState::Playing)))
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
    pub passed: bool,
}

impl Moai {
    fn new(x: f32, height: f32) -> Self {
        Self {
            x,
            height,
            passed: false,
        }
    }
}

#[derive(Resource)]
struct MoaiTexture {
    head: Handle<Image>,
    body: Handle<Image>,
}

impl MoaiTexture {
    fn new(head: Handle<Image>, body: Handle<Image>) -> MoaiTexture {
        MoaiTexture { head, body }
    }
}

fn load_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    let head = asset_server.load("moai.png");
    let body = asset_server.load("moai-segment.png");
    commands.insert_resource(MoaiTexture::new(head, body));
}

fn spawn_moai_body_sprites(parent: &mut ChildBuilder, texture: Handle<Image>) {
    for i in 1..=consts::MOAI_BODY_SEGMENTS_COUNT {
        parent.spawn(SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: consts::MOAI_WIDTH,
                    y: consts::MOAI_HEIGHT,
                }),
                ..default()
            },
            transform: Transform::from_xyz(
                0.0,
                (1.0 - consts::MOAI_BODY_SEGMENTS_OVERLAP_RATIO) * consts::MOAI_HEIGHT * -i as f32,
                -1.0 * i as f32,
            ),
            ..default()
        });
    }
}

fn spawn_moai(mut commands: Commands, moai_texture: Res<MoaiTexture>, x: f32) {
    let mut rng = rand::thread_rng();

    commands
        .spawn((
            SpatialBundle::default(),
            Moai::new(x, rng.gen_range(consts::MOAI_HEIGHT_RANGE.clone())),
            Name::new("Moai"),
        ))
        .with_children(|parent| {
            //down
            parent
                .spawn(SpriteBundle {
                    texture: moai_texture.head.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: consts::MOAI_WIDTH,
                            y: consts::MOAI_HEIGHT,
                        }),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| spawn_moai_body_sprites(parent, moai_texture.body.clone()));

            //up
            parent
                .spawn(SpriteBundle {
                    texture: moai_texture.head.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: consts::MOAI_WIDTH,
                            y: consts::MOAI_HEIGHT,
                        }),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3 {
                        x: 0.0,
                        y: consts::MOAI_HEIGHT + consts::MOAI_VERTICAL_DISTANCE,
                        z: 0.0,
                    })
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    ..default()
                })
                .with_children(|parent| spawn_moai_body_sprites(parent, moai_texture.body.clone()));
        });
}

fn spawn_init_moai(commands: Commands, game_size: Res<GameSize>, moai_texture: Res<MoaiTexture>) {
    let mut x = if game_size.max_x < game_size.max_y {
        game_size.max_x
    } else {
        game_size.max_y
    } * 3.0;
    if x < game_size.max_x + consts::MOAI_WIDTH {
        x = game_size.max_x + consts::MOAI_WIDTH;
    }
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
        transform.translation.y = moai.height - consts::MOAI_HEIGHT / 2.0;
    }
}

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
        let after = before - consts::MOAI_MOVE_SPEED * time.delta_seconds();

        if before >= 0.0 && after <= 0.0 && !moai.passed {
            game_score.inc_score();
            moai.passed = true;
        }

        moai.x = after;
        if moai.x > max_x {
            max_x = moai.x;
        }
    }

    if game_size.max_x + consts::MOAI_WIDTH - max_x >= consts::MOAI_HORIZONTAL_DISTANCE {
        let x = game_size.max_x + consts::MOAI_WIDTH;
        spawn_moai(commands, moai_texture, x);
    }
}

fn despawn_moai_outside_screen(
    mut commands: Commands,
    game_size: Res<GameSize>,
    query_all_moai: Query<(Entity, &Moai)>,
) {
    for (entity, moai) in query_all_moai.iter() {
        if moai.x < game_size.min_x - consts::MOAI_WIDTH {
            commands.entity(entity).despawn_recursive();
        }
    }
}
