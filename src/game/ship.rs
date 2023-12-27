use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::state::GameState;

use super::utils::OnGameScreen;

const SHIP_LOCATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SHIP_SIZE: Vec2 = Vec2::new(25.0, 30.0);

const SHIP_ACCELERATION: f32 = 75.0;
const SHIP_TURN_SPEED: f32 = 0.2;

#[derive(Component)]
pub(super) struct Ship;

#[derive(Bundle)]
struct ShipBundle {
    transform: TransformBundle,
    ship: Ship,
    controller: RigidBody,
    collider: Collider,
    mass: ColliderMassProperties,
    force: ExternalForce,
    gravity: GravityScale,
    damping: Damping,
    sleeping: Sleeping,
    ccd: Ccd,
    on_screen: OnGameScreen,
}

impl ShipBundle {
    fn new(location: Vec3) -> Self {
        Self {
            transform: TransformBundle::from(Transform::from_translation(location)),
            ship: Ship,
            controller: RigidBody::Dynamic,
            collider: Collider::compound(vec![
                (
                    // collider position
                    Vec2::new(0.0, 0.0),
                    // collider rotation
                    0.0,
                    // collider shape
                    Collider::triangle(
                        SHIP_SIZE * Vec2::new(0.0, 0.5),
                        SHIP_SIZE * Vec2::new(0.0, -0.3),
                        SHIP_SIZE * Vec2::new(-0.5, -1.0),
                    ),
                ),
                (
                    // collider position
                    Vec2::new(0.0, 0.0),
                    // collider rotation
                    0.0,
                    // collider shape
                    Collider::triangle(
                        SHIP_SIZE * Vec2::new(0.0, 0.5),
                        SHIP_SIZE * Vec2::new(0.0, -0.3),
                        SHIP_SIZE * Vec2::new(0.5, -1.0),
                    ),
                ),
            ]),
            mass: ColliderMassProperties::Density(1.0),
            force: ExternalForce::default(),
            gravity: GravityScale(0.0),
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 5.0,
            },
            sleeping: Sleeping::disabled(),
            ccd: Ccd::enabled(),
            on_screen: OnGameScreen,
        }
    }
}

fn ship_acceleration_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Transform), With<Ship>>,
) {
    let (mut controller, transform) = query.single_mut();

    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // apply the force in the ship's forward looking direction
    controller.force = movement_factor * transform.up().xy() * SHIP_ACCELERATION;
}

fn ship_turn_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut ExternalForce, With<Ship>>,
) {
    let mut controller = query.single_mut();

    let mut rotation_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    controller.torque = rotation_factor * SHIP_TURN_SPEED;
}

fn setup(mut commands: Commands) {
    let ship = ShipBundle::new(SHIP_LOCATION);
    commands.spawn(ship);
}

pub(super) struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                FixedUpdate,
                (ship_turn_system, ship_acceleration_system).run_if(in_state(GameState::Game)),
            );
    }
}
