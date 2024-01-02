use bevy::prelude::*;
use bevy::window::WindowResized;

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

pub struct GameSizeChangePlugin {
    width: f32,
    height: f32,
}

impl Plugin for GameSizeChangePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSize::new(self.width, self.height))
            .add_systems(Update, on_resize);
    }
}

impl GameSizeChangePlugin {
    pub fn new(width: f32, height: f32) -> GameSizeChangePlugin {
        GameSizeChangePlugin { width, height }
    }
}

fn on_resize(mut resize_reader: EventReader<WindowResized>, mut game_size: ResMut<GameSize>) {
    let e = resize_reader.read().last();
    if let Some(e) = e {
        game_size.update(e.width, e.height);
    }
}
