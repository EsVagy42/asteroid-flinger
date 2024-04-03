use bevy::prelude::*;
use crate::game::components::Position;
use crate::game::wrap;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn collides(&self, position: &Position, other: &CircleCollider, other_position: &Position) -> bool {
        wrap::wrap_vec2(position.0 - other_position.0).length_squared() <= (self.radius + other.radius).powi(2)
    }
}