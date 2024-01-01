use bevy::prelude::*;

#[derive(Debug, Resource, Clone, Copy)]
pub struct GameSize {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl GameSize {
    pub fn new(width: f32, height: f32) -> GameSize {
        GameSize {
            min_x: -width / 2.0,
            max_x: width / 2.0,
            min_y: -height / 2.0,
            max_y: height / 2.0,
        }
    }
}
