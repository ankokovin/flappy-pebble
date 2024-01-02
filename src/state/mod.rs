pub mod gamescore;
pub mod gamestate;

pub struct StatePlugin;

impl bevy::app::PluginGroup for StatePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(gamestate::GameStatePlugin)
            .add(gamescore::GameScorePlugin)
    }
}
