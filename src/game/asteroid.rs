use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;

use crate::utils::state::GameState;

use super::utils::OnGameScreen;

const ASTEROID_FREQUENCY: f32 = 0.5;

#[derive(Component)]
struct Asteroid;

#[derive(Bundle)]
struct AsteroidBundle {
    transform: TransformBundle,
    asteroid: Asteroid,
    body: RigidBody,
    collider: Collider,
    active: ActiveEvents,
    velocity: Velocity,
    gravity: GravityScale,
    sleeping: Sleeping,
    ccd: Ccd,
    on_screen: OnGameScreen,
}

impl AsteroidBundle {
    fn new(location: Vec3, velocity: Vec2) -> Self {
        Self {
            transform: TransformBundle::from_transform(Transform::from_translation(location)),
            asteroid: Asteroid,
            body: RigidBody::Dynamic,
            collider: Collider::ball(25.0),
            active: ActiveEvents::COLLISION_EVENTS,
            velocity: Velocity {
                linvel: velocity,
                angvel: 0.1,
            },
            gravity: GravityScale(0.0),
            sleeping: Sleeping::disabled(),
            ccd: Ccd::enabled(),
            on_screen: OnGameScreen,
        }
    }
}

#[derive(Resource)]
struct AsteroidSpawnConfig {
    timer: Timer,
}

impl Default for AsteroidSpawnConfig {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / ASTEROID_FREQUENCY),
                TimerMode::Repeating,
            ),
        }
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    window_query: Query<&Window>,
    time: Res<Time>,
    mut config: ResMut<AsteroidSpawnConfig>,
) {
    config.timer.tick(time.delta());

    if config.timer.just_finished() {
        let window = window_query.get_single().unwrap();
        let choices = vec![-1.0, 1.0];

        // TODO: since we only have -1.0 and 1.0, the asteroids will only spawn in the corners
        let width = window.width();
        let width_modifier = choices.choose(&mut rand::thread_rng()).unwrap();

        let height = window.height();
        let height_modifier = choices.choose(&mut rand::thread_rng()).unwrap();

        let location = Vec3::new(
            (width / 2.0 + 50.0) * width_modifier,
            (height / 2.0 + 50.0) * height_modifier,
            0.0,
        );

        // TODO: right now the asteroids directly target the center, it would be good to add a
        // random factor to that by picking a point in a x pixel radius of the center
        let velocity = Vec2::new(0.0, 0.0) - location.xy() / 2.0;
        commands.spawn(AsteroidBundle::new(location, velocity));
    }
}

fn setup(mut commands: Commands) {
    commands.init_resource::<AsteroidSpawnConfig>();
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<AsteroidSpawnConfig>();
}

pub(super) struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                FixedUpdate,
                (spawn_asteroid).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), teardown);
    }
}
