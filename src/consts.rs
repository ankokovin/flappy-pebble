pub const WINDOW_HEIGHT: f32 = 1024.0;
pub const WINDOW_WIDTH: f32 = 3072.0;
pub const WINDOW_NAME: &str = "Flappy Pebble :D";
pub const PEBBLE_WIDTH: f32 = 90.0;
pub const PEBBLE_HEIGHT: f32 = 52.0;
pub const PEBBLE_START_Y_RANGE: std::ops::Range<f32> = -300.0..300.0;
pub const PEBBLE_DEFAULT_VELOCITY: f32 = 400.0;
pub const G_FORCE_ACCELERATION: f32 = -400.0;
pub const MOAI_WIDTH: f32 = 100.0;
pub const MOAI_HEIGHT: f32 = 197.2;
pub const MOAI_BODY_SEGMENTS_OVERLAP_RATIO: f32 = 0.1;

//FIXME: this probably should be just calculated based on window size
//       but it would require respawning when window's size change,
//       which I was too lazy to implement.
const MAX_SUPPORTED_HEIGHT: f32 = 4320.0;
pub const MOAI_BODY_SEGMENTS_COUNT: i32 = (MAX_SUPPORTED_HEIGHT / MOAI_HEIGHT + 0.5) as i32;

pub const MOAI_HORIZONTAL_DISTANCE: f32 = 800.0;
pub const MOAI_VERTICAL_DISTANCE: f32 = 300.0;
pub const MOAI_HEIGHT_RANGE: std::ops::Range<f32> = -200.0..200.0;
pub const MOAI_MOVE_SPEED: f32 = 200.0;
