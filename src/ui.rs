use bevy::prelude::*;

use crate::gamestate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_dialog)
            .add_systems(
                Update,
                restart_interaction_system.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_dialog);
    }
}

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct GameOverDialog;

fn spawn_game_over_dialog(mut commands: Commands) {
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
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 50.0,
                    ..Default::default()
                },
            ));
            parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::GRAY),
                        ..Default::default()
                    },
                    RestartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "RESTART",
                        TextStyle {
                            font_size: 35.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn restart_interaction_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interation in interaction_query.iter() {
        if *interation == Interaction::Pressed {
            next_state.set(GameState::Playing);
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
