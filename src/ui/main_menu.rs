use crate::state::gamestate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

#[cfg(not(target_family = "wasm"))]
fn update_systems() -> impl IntoSystemConfigs<()> {
    (start_button_interaction, exit_button_interaction).run_if(in_state(GameState::MainMenu))
}

#[cfg(target_family = "wasm")]
fn update_systems() -> impl IntoSystemConfigs<()> {
    (start_button_interaction).run_if(in_state(GameState::MainMenu))
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, update_systems())
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}

#[derive(Debug, Component)]
struct MainMenu;

#[derive(Debug, Component)]
struct StartGameButton;

#[derive(Debug, Component)]
struct ExitButton;

fn spawn_main_menu(mut commands: Commands) {
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
            MainMenu,
            Name::new("MainMenuContainer"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Flappy Pebbles :D",
                    TextStyle {
                        font_size: 60.0,
                        ..default()
                    },
                ),
                Name::new("MainMenuTitle"),
            ));
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(50.0)),
                            row_gap: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("MainMenuButtonsContainer"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: BackgroundColor(Color::GRAY),
                                style: Style {
                                    padding: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            StartGameButton,
                            Name::new("StartGameButton"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Start game",
                                    TextStyle {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                ),
                                Name::new("StartGameButtonLabel"),
                            ));
                        });

                    if !cfg!(target_family = "wasm") {
                        parent
                            .spawn((
                                ButtonBundle {
                                    background_color: BackgroundColor(Color::GRAY),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(20.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ExitButton,
                                Name::new("ExitButton"),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "Exit",
                                        TextStyle {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ),
                                    Name::new("ExitButtonLabel"),
                                ));
                            });
                    }
                });
        });
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for menu in query.iter() {
        commands.entity(menu).despawn_recursive();
    }
}

fn start_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn exit_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            exit.send(bevy::app::AppExit);
        }
    }
}
