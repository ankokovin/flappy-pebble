use bevy::prelude::*;

use crate::state::{gamescore::GameScore, gamestate::GameState};

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                from: GameState::MainMenu,
                to: GameState::Playing,
            },
            spawn_scoreboard,
        )
        .add_systems(
            OnTransition {
                from: GameState::GameOver,
                to: GameState::Playing,
            },
            spawn_scoreboard,
        )
        .add_systems(OnEnter(GameState::MainMenu), despawn_scoreboard)
        .add_systems(OnEnter(GameState::GameOver), despawn_scoreboard)
        .add_systems(
            Update,
            (update_scoreboard, update_is_highscore_label).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component)]
struct ScoreBoard;

#[derive(Debug, Component)]
struct ScoreLabel;

#[derive(Debug, Component)]
struct IsHighScoreLabel;

fn spawn_scoreboard(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            },
            ScoreBoard,
            Name::new("ScoreBoard"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        //TODO: I probably wanted to change something in style
                        //      (specifically flex) but forgot to.
                        style: Style { ..default() },
                        ..default()
                    },
                    Name::new("ScoreLabelContainer"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Score: ",
                            TextStyle {
                                font_size: 20.0,
                                ..default()
                            },
                        ),
                        Name::new("ScoreLabelLabel"),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "<score>",
                            TextStyle {
                                font_size: 20.0,
                                ..default()
                            },
                        ),
                        ScoreLabel,
                        Name::new("ScoreLabel"),
                    ));
                });

            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::RED,
                        ..default()
                    },
                ),
                IsHighScoreLabel,
            ));
        });
}

fn despawn_scoreboard(mut commands: Commands, query_scoreboard: Query<Entity, With<ScoreBoard>>) {
    for entity in query_scoreboard.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

//TODO: could this run on event and not on update?
fn update_scoreboard(
    game_score: Res<GameScore>,
    mut query_score_label: Query<&mut Text, With<ScoreLabel>>,
) {
    let text = query_score_label.get_single_mut();
    if let Err(error) = text {
        debug!("{}", error);
        return;
    }
    let mut text = text.unwrap();
    let section = text.sections.first_mut().expect("to have a TextSection");
    section.value = game_score.get_current().to_string();
}

fn update_is_highscore_label(
    game_score: Res<GameScore>,
    mut query_is_highscore_label: Query<&mut Text, With<IsHighScoreLabel>>,
) {
    let text = query_is_highscore_label.get_single_mut();
    if let Err(error) = text {
        debug!("{}", error);
        return;
    }
    let mut text = text.unwrap();
    let section = text.sections.first_mut().expect("to have a TextSection");
    section.value = (if game_score.is_new_high_score() {
        "Highscore!"
    } else {
        ""
    })
    .to_string();
}
