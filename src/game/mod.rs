mod background;
mod ship;
mod utils;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{background::BackgroundPlugin, utils::DespawnerPlugin};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        // initialize plugin group
        PluginGroupBuilder::start::<Self>()
            // add plugins to the group
            .add(BackgroundPlugin)
            .add(DespawnerPlugin)
    }
}
