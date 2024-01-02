mod game_over_dialog;
pub struct UiPlugin;

impl bevy::app::PluginGroup for UiPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(game_over_dialog::GameOverDialogPlugin)
    }
}
