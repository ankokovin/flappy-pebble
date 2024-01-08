use crate::state::gamestate::GameState;
use crate::ui::buttons::{change_state_button, ChangeStateButton, DEFAULT_BUTTON_COLOR};
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                PauseButton::keyboard_pressed_system,
                PauseButton::gamepad_button_pressed_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                UnpauseButton::button_pressed_system,
                UnpauseButton::keyboard_pressed_system,
                UnpauseButton::gamepad_button_pressed_system,
            )
                .run_if(in_state(GameState::Pause)),
        )
        .add_systems(OnEnter(GameState::Pause), spawn_game_pause_menu)
        .add_systems(OnExit(GameState::Pause), despawn_game_pause_menu);
    }
}

#[derive(Debug, Component)]
struct PauseMenu;

#[derive(Debug, Component, ChangeStateButton)]
#[target_state(Pause)]
#[keyboard(Escape)]
#[gamepad(East)]
struct PauseButton;

#[derive(Debug, Component, ChangeStateButton)]
#[target_state(Playing)]
#[keyboard(Escape, Return)]
#[gamepad(East, North)]
struct UnpauseButton;

fn spawn_game_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::new(
                        Val::Px(50.0),
                        Val::Px(50.0),
                        Val::Px(100.0),
                        Val::Px(50.0),
                    ),
                    ..default()
                },
                ..default()
            },
            PauseMenu,
            Name::new("PauseDialog"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "PAUSED",
                    TextStyle {
                        font_size: 50.0,
                        ..default()
                    },
                ),
                Name::new("PauseDialogText"),
            ));
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
                    UnpauseButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "CONTINUE",
                            TextStyle {
                                font_size: 35.0,
                                ..default()
                            },
                        ),
                        Name::new("UnpauseButtonText"),
                    ));
                });
        });
}

fn despawn_game_pause_menu(mut commands: Commands, dialog_query: Query<Entity, With<PauseMenu>>) {
    for dialog in dialog_query.iter() {
        commands.entity(dialog).despawn_recursive();
    }
}
