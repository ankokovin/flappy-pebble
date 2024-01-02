use bevy::prelude::*;
use rand::Rng;

use crate::consts::Consts;
use crate::{game_size::GameSize, state::gamescore::GameScore, state::gamestate::GameState};

pub struct MoaiPlugin;

impl Plugin for MoaiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Moai>()
            .add_systems(Startup, load_texture)
            .add_systems(
                OnEnter(GameState::Playing),
                (despawn_all_moai, spawn_init_moai).chain(),
            )
            .add_systems(OnEnter(GameState::MainMenu), despawn_all_moai)
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

fn spawn_moai(mut commands: Commands, moai_texture: Res<MoaiTexture>, consts: Res<Consts>, x: f32) {
    let mut rng = rand::thread_rng();
    commands
        .spawn((
            SpatialBundle::default(),
            Moai {
                x,
                height: rng.gen_range(consts.moai_height_range.clone()),
            },
            Name::new("Moai"),
        ))
        .with_children(|parent| {
            //down
            parent.spawn(SpriteBundle {
                texture: moai_texture.handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: consts.moai_width,
                        y: consts.moai_height,
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
                        x: consts.moai_width,
                        y: consts.moai_height,
                    }),
                    flip_x: true,
                    flip_y: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3 {
                    x: 0.0,
                    y: consts.moai_height + consts.moai_vertical_distance,
                    z: 0.0,
                }),
                ..Default::default()
            });
        });
}

fn spawn_init_moai(
    commands: Commands,
    game_size: Res<GameSize>,
    moai_texture: Res<MoaiTexture>,
    consts: Res<Consts>,
) {
    let mut x = if game_size.max_x < game_size.max_y {
        game_size.max_x
    } else {
        game_size.max_y
    } * 3.0;
    if x < game_size.max_x + consts.moai_width {
        x = game_size.max_x + consts.moai_width;
    }
    spawn_moai(commands, moai_texture, consts, x)
}

fn despawn_all_moai(mut commands: Commands, query_all_moai: Query<Entity, With<Moai>>) {
    for moai in query_all_moai.iter() {
        commands.entity(moai).despawn_recursive();
    }
}

fn render_moai(mut query_all_moai: Query<(&Moai, &mut Transform)>, consts: Res<Consts>) {
    for (moai, mut transform) in query_all_moai.iter_mut() {
        transform.translation.x = moai.x;
        transform.translation.y = moai.height - consts.moai_height / 2.0;
    }
}

fn move_moai(
    time: Res<Time<Fixed>>,
    mut query_all_moai: Query<&mut Moai>,
    commands: Commands,
    game_size: Res<GameSize>,
    moai_texture: Res<MoaiTexture>,
    consts: Res<Consts>,
    mut game_score: ResMut<GameScore>,
) {
    let mut max_x = f32::MIN;
    for mut moai in query_all_moai.iter_mut() {
        let before = moai.x;
        let after = before - consts.moai_move_speed * time.delta_seconds();

        if before >= 0.0 && after <= 0.0 {
            game_score.inc_score();
        }

        moai.x = after;
        if moai.x > max_x {
            max_x = moai.x;
        }
    }

    if game_size.max_x + consts.moai_width - max_x >= consts.moai_horizontal_distance {
        let x = game_size.max_x + consts.moai_width;
        spawn_moai(commands, moai_texture, consts, x);
    }
}

fn despawn_moai_outside_screen(
    mut commands: Commands,
    game_size: Res<GameSize>,
    consts: Res<Consts>,
    query_all_moai: Query<(Entity, &Moai)>,
) {
    for (entity, moai) in query_all_moai.iter() {
        if moai.x < game_size.min_x - consts.moai_width {
            commands.entity(entity).despawn_recursive();
        }
    }
}
