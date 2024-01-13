use std::ops::RangeBounds;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;

use crate::consts;

#[derive(Debug, Resource, Clone, Copy, Default)]
pub struct GameSize {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl GameSize {
    fn new(width: f32, height: f32) -> GameSize {
        let mut size = GameSize::default();
        size.update(width, height);
        size
    }

    fn update(&mut self, width: f32, height: f32) {
        self.min_x = -width / 2.0;
        self.max_x = width / 2.0;
        self.min_y = -height / 2.0;
        self.max_y = height / 2.0;
    }
}

pub struct GameSizePlugin {
    width: f32,
    height: f32,
}

impl Plugin for GameSizePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSize::new(self.width, self.height))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, on_resize);
    }
}

impl GameSizePlugin {
    pub fn new(width: f32, height: f32) -> GameSizePlugin {
        GameSizePlugin { width, height }
    }
}

#[derive(Debug, Component)]
struct MyCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MyCamera));
}

fn on_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut game_size: ResMut<GameSize>,
    mut query_camera: Query<&mut OrthographicProjection, With<MyCamera>>,
) {
    let e = resize_reader.read().last();
    if let Some(e) = e {
        let physical_width = e.width;
        let physical_height = e.height;

        let minimal_logical_width = get_minimal_logival_width();
        let scale_based_on_width = calculate_scale(minimal_logical_width, physical_width);

        let minimal_logical_height = get_minimal_logical_height();
        let scale_based_on_height = calculate_scale(minimal_logical_height, physical_height);

        //Finally, take the minimum and apply.
        let scale = f32::min(scale_based_on_width, scale_based_on_height);

        //New game size is equals to physical window size scaled backwards
        let logical_width = physical_width / scale;
        let logical_height = physical_height / scale;

        //Apply scaling by changing camera scaling
        let mut projection = query_camera.single_mut();
        projection.scaling_mode = ScalingMode::WindowSize(scale);

        game_size.update(logical_width, logical_height);
    }
}

//Let's define a rule: we should be able to see next moai.
fn get_minimal_logival_width() -> f32 {
    consts::MOAI_WIDTH * 2.0 + consts::MOAI_HORIZONTAL_DISTANCE
}

//And another rule: we should be able to see both top and bottom moai.
fn get_minimal_logical_height() -> f32 {
    let moai_height_range_end = get_bound_or(consts::MOAI_HEIGHT_RANGE.end_bound(), 0.0);
    let moai_height_range_start = get_bound_or(consts::MOAI_HEIGHT_RANGE.start_bound(), 0.0);
    consts::MOAI_HEIGHT * 2.0
        + (moai_height_range_end - moai_height_range_start)
        + consts::MOAI_VERTICAL_DISTANCE
}

fn calculate_scale(minimal_logical: f32, physical: f32) -> f32 {
    if physical < minimal_logical {
        physical / minimal_logical
    } else {
        1.0
    }
}

fn get_bound_or(bound: std::ops::Bound<&f32>, default: f32) -> f32 {
    match bound {
        std::ops::Bound::Excluded(val) => *val,
        std::ops::Bound::Included(val) => *val,
        _ => default,
    }
}
