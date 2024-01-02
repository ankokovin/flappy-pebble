use bevy::prelude::*;

use crate::state::{gamescore::GameScore, gamestate::GameState};

pub struct GameOverDialogPlugin;

impl Plugin for GameOverDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_dialog)
            .add_systems(
                Update,
                (
                    restart_button_interaction,
                    main_menu_button_interation,
                    player_input,
                )
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_dialog);
    }
}

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct MainMenuButton;

#[derive(Component)]
struct GameOverDialog;

fn spawn_game_over_dialog(mut commands: Commands, game_score: Res<GameScore>) {
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
            GameOverDialog,
            Name::new("GameOverDialog"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font_size: 50.0,
                        ..Default::default()
                    },
                ),
                Name::new("GameOverDialogText"),
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Score: ".to_string() + &game_score.get_current().to_string(),
                    TextStyle {
                        font_size: 30.0,
                        ..Default::default()
                    },
                ),
                Name::new("GameScoreLabel"),
            ));
            if game_score.is_new_high_score() {
                parent.spawn((
                    TextBundle::from_section(
                        "New high score!",
                        TextStyle {
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ),
                    Name::new("NewHighScoreLabel"),
                ));
            } else {
                parent.spawn((
                    TextBundle::from_section(
                        "Highscore: ".to_string() + &game_score.get_best().to_string(),
                        TextStyle {
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ),
                    Name::new("HighScoreLabel"),
                ));
            }

            parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::GRAY),
                        style: Style {
                            padding: UiRect::all(Val::Px(20.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    RestartButton,
                    Name::new("RestartButton"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "RESTART",
                            TextStyle {
                                font_size: 35.0,
                                ..Default::default()
                            },
                        ),
                        Name::new("RestartButtonText"),
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::GRAY),
                        style: Style {
                            padding: UiRect::all(Val::Px(20.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MainMenuButton,
                    Name::new("MainMenuButton"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "MAIN MENU",
                            TextStyle {
                                font_size: 35.0,
                                ..Default::default()
                            },
                        ),
                        Name::new("MainMenuButtonText"),
                    ));
                });
        });
}

fn restart_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

fn main_menu_button_interation(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}

fn despawn_game_over_dialog(
    mut commands: Commands,
    dialog_query: Query<Entity, With<GameOverDialog>>,
) {
    for dialog in dialog_query.iter() {
        commands.entity(dialog).despawn_recursive();
    }
}

fn player_input(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.any_just_pressed(vec![KeyCode::Space, KeyCode::Return]) {
        next_state.set(GameState::Playing);
    }

    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
