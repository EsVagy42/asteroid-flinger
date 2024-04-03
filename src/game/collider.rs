use bevy::prelude::*;
use crate::game::components::Position;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn collides(&self, position: &Position, other: &CircleCollider, other_position: &Position) -> bool {
        position.0.distance_squared(other_position.0) <= (self.radius + other.radius).powi(2)
    }
}