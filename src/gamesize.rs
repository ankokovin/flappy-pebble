use bevy::prelude::*;

#[derive(Debug, Resource, Clone, Copy)]
pub struct GameSize {
    pub min_y: f32,
    pub max_y: f32,
}
