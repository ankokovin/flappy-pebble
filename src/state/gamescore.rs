use bevy::prelude::*;

use super::gamestate::GameState;
use crate::consts::HIGHSCORE_PATH;

type Score = u32;

#[derive(Debug, Resource, Default)]
pub struct GameScore {
    current_score: Score,
    high_score: Score,
    is_high_score: bool,
}

impl GameScore {
    pub fn inc_score(&mut self) {
        self.current_score += 1;
        if self.current_score > self.high_score {
            self.is_high_score = true;
        }
    }
    pub fn get_current_score(&self) -> Score {
        self.current_score
    }
    pub fn get_high_score(&self) -> Score {
        self.high_score
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
    game_score.current_score = 0;
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
        game_score.high_score = game_score.current_score;
        save_highscore(game_score.high_score);
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
    score.high_score = read_highscore;
}

#[cfg(target_family = "wasm")]
fn load_highscore(mut score: ResMut<GameScore>) {
    use gloo_storage::{LocalStorage, Storage};

    score.high_score = LocalStorage::get(HIGHSCORE_PATH).unwrap_or_else(|_| {
        warn!("Could not read highscore");
        Score::default()
    });
}
