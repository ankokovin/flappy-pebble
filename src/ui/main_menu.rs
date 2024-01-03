use crate::state::gamestate::GameState;
use crate::ui::buttons::{change_state_button, ChangeStateButton, DEFAULT_BUTTON_COLOR};
use bevy::prelude::*;

pub struct MainMenuPlugin;

#[cfg(not(target_family = "wasm"))]
fn update_systems() -> impl IntoSystemConfigs<()> {
    (
        StartGameButton::button_pressed_system,
        StartGameButton::keyboard_pressed_system,
        StartGameButton::gamepad_button_pressed_system,
        ExitButton::button_pressed_system,
    )
        .run_if(in_state(GameState::MainMenu))
}

#[cfg(target_family = "wasm")]
fn update_systems() -> impl IntoSystemConfigs<()> {
    (
        StartGameButton::button_pressed_system,
        StartGameButton::keyboard_pressed_system,
        StartGameButton::gamepad_button_pressed_system,
    )
        .run_if(in_state(GameState::MainMenu))
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

#[derive(Debug, Component, ChangeStateButton)]
#[target_state(Playing)]
#[keyboard(Return)]
#[gamepad(South)]
struct StartGameButton;

#[derive(Debug, Component, ChangeStateButton)]
#[target_state(Exit)]
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
                        .spawn(change_state_button(
                            ButtonBundle {
                                background_color: DEFAULT_BUTTON_COLOR.into(),
                                style: Style {
                                    padding: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            StartGameButton,
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
                            .spawn(change_state_button(
                                ButtonBundle {
                                    background_color: DEFAULT_BUTTON_COLOR.into(),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(20.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ExitButton,
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
