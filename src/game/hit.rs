use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::state::GameState;

fn hit_registration_system(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        let (entity1, entity2, flags) = match collision_event {
            CollisionEvent::Started(e1, e2, flags) => (e1, e2, flags),
            CollisionEvent::Stopped(e1, e2, flags) => (e1, e2, flags),
        };

        println!(
            "entity 1: {:?}, entity 2: {:?}, flags: {:?}",
            entity1, entity2, flags
        );
    }
}

pub(super) struct HitPlugin;

impl Plugin for HitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (hit_registration_system).run_if(in_state(GameState::Game)),
        );
    }
}
