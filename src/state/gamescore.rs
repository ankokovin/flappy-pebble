use bevy::prelude::*;

use super::gamestate::GameState;

type Score = u32;

#[derive(Debug, Resource, Default)]
pub struct GameScore {
    current_value: Score,
    best_value: Score,
    is_high_score: bool,
}

impl GameScore {
    pub fn inc_score(&mut self) {
        self.current_value += 1;
        if self.current_value > self.best_value {
            self.is_high_score = true;
        }
    }

    fn update_high_score(&mut self) {
        debug_assert!(self.current_value > self.best_value);
        self.best_value = self.current_value;
    }

    pub fn get_current(&self) -> Score {
        self.current_value
    }

    pub fn get_best(&self) -> Score {
        self.best_value
    }
    pub fn is_new_high_score(&self) -> bool {
        self.is_high_score
    }
}

pub struct GameScorePlugin;

impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_systems(OnEnter(GameState::Playing), reset_score);
    }
}

fn reset_score(mut game_score: ResMut<GameScore>) {
    if game_score.is_high_score {
        game_score.update_high_score();
    }
    game_score.current_value = 0;
    game_score.is_high_score = false;
}
