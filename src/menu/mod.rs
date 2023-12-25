mod actions;
mod setup;

use bevy::prelude::*;

use crate::utils::state::GameState;

use actions::{button_system, menu_action};
use setup::setup;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(
                Update,
                (button_system, menu_action).run_if(in_state(GameState::Menu)),
            );
    }
}
