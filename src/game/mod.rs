mod background;
mod utils;

use bevy::{app::PluginGroupBuilder, prelude::*};

use background::BackgroundPlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        // initialize plugin group
        PluginGroupBuilder::start::<Self>()
            // add plugins to the group
            .add(BackgroundPlugin)
    }
}
