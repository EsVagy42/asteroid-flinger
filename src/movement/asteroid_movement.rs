use bevy::prelude::*;
use crate::game::components::{Acceleration, Position};
use crate::player::Player;

use super::MovementSchedule;

#[derive(Component)]
pub struct AsteroidMovement {
    pub gravity_multiplier: f32,
    pub repulsion_multiplier: f32,
}

fn apply(
    mut query: Query<(&AsteroidMovement, &Position, &mut Acceleration)>,
    player_query: Query<&Position, With<Player>>,   
) {
    let player_position = player_query.single();
    let (asteroid_movement, position, mut acceleration) = query.single_mut();
    let direction = player_position.0 - position.0;
    acceleration.0 = direction * asteroid_movement.gravity_multiplier;
    let direction = direction.normalize_or_zero();
    acceleration.0 -= direction * asteroid_movement.repulsion_multiplier;
}

pub struct AsteroidMovementPlugin;

impl Plugin for AsteroidMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(MovementSchedule, apply);
    }
}