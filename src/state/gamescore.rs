use bevy::prelude::*;

use super::gamestate::GameState;
use crate::consts::HIGHSCORE_PATH;

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
            .add_systems(Startup, load_highscore)
            .add_systems(OnEnter(GameState::GameOver), handle_highscore)
            .add_systems(OnExit(GameState::GameOver), reset_score);
    }
}

fn reset_score(mut game_score: ResMut<GameScore>) {
    game_score.current_value = 0;
    game_score.is_high_score = false;
}

#[cfg(not(target_family = "wasm"))]
fn save_highscore(highscore: Score) {
    use std::{fs::File, io::Write};

    use bevy::tasks::IoTaskPool;

    IoTaskPool::get()
        .spawn(async move {
            File::create(format!("assets/{HIGHSCORE_PATH}"))
                .and_then(|mut file| file.write(&highscore.to_be_bytes()))
                .unwrap_or_else(|_| {
                    warn!("Could not write highscore");
                    0
                });
        })
        .detach();
}

fn handle_highscore(mut game_score: ResMut<GameScore>) {
    if game_score.is_high_score {
        game_score.best_value = game_score.current_value;
        save_highscore(game_score.best_value);
    }
}

#[cfg(target_family = "wasm")]
fn save_highscore(highscore: Score) {
    use gloo_storage::{LocalStorage, Storage};

    LocalStorage::set(HIGHSCORE_PATH, highscore).unwrap_or_else(|_| {
        warn!("Could not save highscore");
    });
}

#[cfg(not(target_family = "wasm"))]
fn load_highscore(mut score: ResMut<GameScore>) {
    use std::{fs::File, io::Read};
    let mut read_highscore: Score = 0;
    let mut buffer = read_highscore.to_be_bytes();
    File::open(format!("assets/{HIGHSCORE_PATH}"))
        .and_then(|mut file| file.read(&mut buffer))
        .unwrap_or_else(|_| {
            warn!("Could not read highscore");
            0
        });
    read_highscore = Score::from_be_bytes(buffer);
    score.best_value = read_highscore;
}

#[cfg(target_family = "wasm")]
fn load_highscore(mut score: ResMut<GameScore>) {
    use gloo_storage::{LocalStorage, Storage};

    score.best_value = LocalStorage::get(HIGHSCORE_PATH).unwrap_or_else(|_| {
        warn!("Could not read highscore");
        Score::default()
    });
}
