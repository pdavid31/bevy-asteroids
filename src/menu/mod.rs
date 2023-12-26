mod actions;
mod setup;

use bevy::prelude::*;

use crate::utils::{despawn::despawn_screen, state::GameState};

use actions::{button_system, menu_action};
use setup::{setup, OnMenuScreen};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // set up the menu when we enter the game state
            .add_systems(OnEnter(GameState::Menu), setup)
            // initialize all menu systems
            .add_systems(
                Update,
                (button_system, menu_action).run_if(in_state(GameState::Menu)),
            )
            // tear down all entities that are related to the menu
            // when we leave the menu state
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
    }
}
