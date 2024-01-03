pub mod gamescore;
pub mod gamestate;
mod input_mode;

pub struct StatePlugin;

impl bevy::app::PluginGroup for StatePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(gamestate::GameStatePlugin)
            .add(gamescore::GameScorePlugin)
    }
}
