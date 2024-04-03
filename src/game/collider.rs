use crate::game::components::Position;
use crate::game::wrap;
use bevy::prelude::*;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn collides(
        &self,
        position: &Position,
        other: &CircleCollider,
        other_position: &Position,
    ) -> bool {
        wrap::wrap_vec2(position.0 - other_position.0).length_squared()
            <= (self.radius + other.radius).powi(2)
    }
}

#[cfg(test)]
mod collider_tests {
    use super::*;

    #[test]
    fn test_collides() {
        assert!(CircleCollider { radius: 1. }.collides(
            &Position(Vec2::new(-1024., -1024.)),
            &CircleCollider { radius: 1. },
            &Position(Vec2::new(1023., 1023.))
        ));
        assert!(!CircleCollider { radius: 1. }.collides(
            &Position(Vec2::new(-1023., -1023.)),
            &CircleCollider { radius: 1. },
            &Position(Vec2::new(1023., 1023.))
        ));
    }
}
