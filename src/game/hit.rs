use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::laser::Laser;
use crate::utils::state::GameState;

fn hit_registration_system(world: &World, mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let is_e1_laser = world.entity(*e1).contains::<Laser>();

                let is_e2_laser = world.entity(*e2).contains::<Laser>();

                println!("e1 is laser: {}, e2 is laser: {}", is_e1_laser, is_e2_laser);
            }
            CollisionEvent::Stopped(e1, e2, _) => {}
        };
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
