use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

const WINDOW_HEIGHT: f32 = 1024.0;
const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * 3.0;
const WINDOW_NAME: &str = "Flappy Pebble :D";

const PEBBLE_WIDTH: f32 = 90.0;
const PEBBLE_HEIGHT: f32 = 52.0;
const PEBBLE_START_Y_RANGE: std::ops::Range<f32> = -300.0..300.0;
const G_FORCE_ACCELERATION: f32 = -400.0;
const PEBBLE_DEFAULT_VELOCITY: f32 = 400.0;

const MOAI_VERTICAL_DISTANCE: f32 = 300.0;
const MOAI_HEIGHT_RANGE: std::ops::Range<f32> = -200.0..200.0;
const MOAI_MOVE_SPEED: f32 = 200.0;
const MOAI_HORIZONTAL_DISTANCE: f32 = 800.0;
const MOAI_WIDTH: f32 = 100.0;
const MOAI_HEIGHT: f32 = 1345.0;

#[derive(Debug, Resource)]
pub struct Consts {
    pub window_height: f32,
    pub window_width: f32,
    pub window_name: &'static str,

    pub pebble_width: f32,
    pub pebble_height: f32,
    pub pebble_start_y_range: std::ops::Range<f32>,
    pub pebble_default_velocity: f32,
    pub g_force_acceleration: f32,

    pub moai_width: f32,
    pub moai_height: f32,
    pub moai_horizontal_distance: f32,
    pub moai_vertical_distance: f32,
    pub moai_height_range: std::ops::Range<f32>,
    pub moai_move_speed: f32,
}

impl Default for Consts {
    fn default() -> Self {
        Consts {
            window_height: WINDOW_HEIGHT,
            window_width: WINDOW_WIDTH,
            window_name: WINDOW_NAME,

            pebble_width: PEBBLE_WIDTH,
            pebble_height: PEBBLE_HEIGHT,
            pebble_start_y_range: PEBBLE_START_Y_RANGE,
            pebble_default_velocity: PEBBLE_DEFAULT_VELOCITY,
            g_force_acceleration: G_FORCE_ACCELERATION,

            moai_height: MOAI_HEIGHT,
            moai_width: MOAI_WIDTH,
            moai_horizontal_distance: MOAI_HORIZONTAL_DISTANCE,
            moai_vertical_distance: MOAI_VERTICAL_DISTANCE,
            moai_height_range: MOAI_HEIGHT_RANGE,
            moai_move_speed: MOAI_MOVE_SPEED,
        }
    }
}

pub struct ConstsPlugin;

impl Plugin for ConstsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Consts::default());
    }
}
