mod menu;
mod utils;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use menu::MenuPlugin;
use utils::{camera::CameraPlugin, state::GameStatePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(MenuPlugin)
        .run();
}
