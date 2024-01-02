use bevy::prelude::*;

use crate::state::{gamescore::GameScore, gamestate::GameState};

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_scoreboard)
            .add_systems(OnExit(GameState::Playing), despawn_scoreboard)
            .add_systems(
                Update,
                update_scoreboard.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Debug, Component)]
struct ScoreBoard;

#[derive(Debug, Component)]
struct ScoreLabel;

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
                    ..Default::default()
                },
                ..Default::default()
            },
            ScoreBoard,
            Name::new("ScoreBoard"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Name::new("ScoreLabelContainer"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Score: ",
                            TextStyle {
                                font_size: 20.0,
                                ..Default::default()
                            },
                        ),
                        Name::new("ScoreLabelLabel"),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "<score>",
                            TextStyle {
                                font_size: 20.0,
                                ..Default::default()
                            },
                        ),
                        ScoreLabel,
                        Name::new("ScoreLabel"),
                    ));
                });
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
    let mut text = query_score_label.single_mut();
    let section = text.sections.first_mut().expect("to have a TextSection");
    section.value = game_score.get_current().to_string();
}
