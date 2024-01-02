mod moai;
mod pebble;

pub struct GameEntityPlugin;

impl bevy::app::PluginGroup for GameEntityPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(pebble::PebblePlugin)
            .add(moai::MoaiPlugin)
    }
}
