use bevy::prelude::*;
use crate::game_components::components::*;
use crate::game_components::input::*;

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