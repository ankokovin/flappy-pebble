mod buttons;
mod game_over_dialog;
mod main_menu;
mod pause_menu;
mod scoreboard;

pub struct UiPlugin;

impl bevy::app::PluginGroup for UiPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(game_over_dialog::GameOverDialogPlugin)
            .add(scoreboard::ScoreBoardPlugin)
            .add(main_menu::MainMenuPlugin)
            .add(pause_menu::PauseMenuPlugin)
    }
}
