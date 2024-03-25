use bevy::{ecs::component::Component, transform::components::Transform};
use crate::wrap::*;

pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct Collider(pub CircleCollider);

pub fn check_collision(transform_a: &Transform, collider_a: &Collider, transform_b: &Transform, collider_b: &Collider) -> bool {
    let distance_sqrd = wrap(transform_a.translation - transform_b.translation).truncate().length_squared();
    return distance_sqrd <= f32::powf(collider_a.0.radius + collider_b.0.radius, 2.);
}