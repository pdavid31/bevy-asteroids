use bevy::prelude::*;

use crate::utils::{despawn::despawn_screen, state::GameState};

#[derive(Component)]
pub(super) struct OnGameScreen;

pub(super) struct DespawnerPlugin;

impl Plugin for DespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}
