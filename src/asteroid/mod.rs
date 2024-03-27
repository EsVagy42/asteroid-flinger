use bevy::prelude::*;
use crate::game_components::{components::*, input::*, wrap::*};
use crate::player::*;

const ASTEROID_GRAVITY_SCALE: f32 = 150.;
const ASTEROID_REPULSION_SCALE: f32 = 4000.;
const INACTIVATION_SPEED_SQRD: f32 = 1.;
pub const ASTEROID_DRAG: f32 = 0.95;
const DETACHED_ASTEROID_DRAG: f32 = 0.5;
const ASTEROID_PICKUP_DISTANCE_SQRD: f32 = 32. * 32.;
const ASTEROID_DETACHMENT_BOOST_MULTIPLIER: f32 = 1.5;

#[derive(Component)]
pub enum Asteroid {
    Attached,
    Detached,
    Inactive
}

pub fn update_asteroid_velocity(
    mut query: Query<(&Asteroid, &Transform, &mut Velocity)>,
    player_query: Query<& Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    for (asteroid, transform, mut velocity) in query.iter_mut() {
        if let Asteroid::Attached = asteroid {
            let mut direction = (player_transform.translation - transform.translation).truncate();
            velocity.0 += direction * ASTEROID_GRAVITY_SCALE * time.delta_seconds();
            direction = direction.normalize_or_zero();
            velocity.0 -= direction * ASTEROID_REPULSION_SCALE * time.delta_seconds();
        }
    }
}

pub fn update_asteroid_state(
    mut asteroid_query: Query<(&mut Asteroid, &Transform, &mut Velocity, &mut Drag)>,
    player_query: Query<&Transform, With<Player>>,
    input: Res<GameInput>,
) {
    let player_transform = player_query.single();
    let just_released = input.just_released;
    for (mut asteroid, transform, mut velocity, mut drag) in asteroid_query.iter_mut() {
        if let Asteroid::Attached = *asteroid {
            if just_released {
                *asteroid = Asteroid::Detached;
                velocity.0 *= ASTEROID_DETACHMENT_BOOST_MULTIPLIER;
                drag.0 = DETACHED_ASTEROID_DRAG;
            }
        } else {
            if wrap(transform.translation - player_transform.translation).truncate().length_squared() < ASTEROID_PICKUP_DISTANCE_SQRD {
                *asteroid = Asteroid::Attached;
                drag.0 = ASTEROID_DRAG;
            }
            
            if let Asteroid::Detached = *asteroid {
                if velocity.0.length_squared() < INACTIVATION_SPEED_SQRD {
                    *asteroid = Asteroid::Inactive;
                }
            }
        }
    }
}
