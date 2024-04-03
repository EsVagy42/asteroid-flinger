use bevy::prelude::*;
use crate::game::components::Acceleration;
use crate::input::GameInput;

use super::MovementSchedule;

#[derive(Component)]
pub struct InputMovement {
    pub speed: f32,
}

pub fn apply(
    mut query: Query<(&InputMovement, &mut Acceleration)>,
    input: Res<GameInput>,
) {
    for (input_movement, mut acceleration) in query.iter_mut() {
        acceleration.0 = input.direction * input_movement.speed;
    }
}

pub struct InputMovementPlugin;

impl Plugin for InputMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(MovementSchedule, apply);
    }
}