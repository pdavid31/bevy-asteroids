use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::state::GameState;

use super::{ship::Ship, utils::OnGameScreen};

const LASER_SPEED: f32 = 1000.0;
// laser firing frequency in hertz
const LASER_FREQUENCY: f32 = 10.0;

#[derive(Component)]
struct Laser;

#[derive(Bundle)]
struct LaserBundle {
    transform: TransformBundle,
    laser: Laser,
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    velocity: Velocity,
    on_screen: OnGameScreen,
}

impl LaserBundle {
    fn new(location: Vec3, rotation: Quat, velocity: Vec2) -> Self {
        Self {
            transform: TransformBundle {
                local: Transform {
                    translation: location,
                    rotation: rotation,
                    ..default()
                },
                ..default()
            },
            laser: Laser,
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(5.0, 10.0),
            sensor: Sensor,
            velocity: Velocity {
                linvel: velocity,
                angvel: 0.0,
            },
            on_screen: OnGameScreen,
        }
    }
}

#[derive(Resource)]
struct LaserTriggerConfig {
    timer: Timer,
}

impl Default for LaserTriggerConfig {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / LASER_FREQUENCY),
                TimerMode::Once,
            ),
        }
    }
}

fn laser_trigger_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&Transform, With<Ship>>,
    time: Res<Time>,
    mut config: ResMut<LaserTriggerConfig>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    // if the timer did not finish yet, do nothing
    if !config.timer.finished() {
        return;
    }

    let transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn(LaserBundle::new(
            transform.translation,
            transform.rotation,
            transform.up().xy().normalize() * LASER_SPEED,
        ));

        config.timer.reset();
    }
}

fn setup(mut commands: Commands) {
    commands.init_resource::<LaserTriggerConfig>();
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<LaserTriggerConfig>();
}

pub(super) struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                FixedUpdate,
                (laser_trigger_system).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), teardown);
    }
}
