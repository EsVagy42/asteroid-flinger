use bevy::prelude::*;
use crate::{general_components::*, player::Player};

const ASTEROID_GRAVITY_SCALE: f32 = 100.;
const ASTEROID_REPULSION_SCALE: f32 = 2500.;

#[derive(Component)]
pub enum Asteroid {
    Attached,
    Detached,
    Inactive
}

pub fn update_asteroid_velocity(
    mut query: Query<(&Asteroid, &Transform, &mut Velocity)>,
    mut player_query: Query<& Transform, With<Player>>,
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