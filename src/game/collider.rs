use crate::game::components::Position;
use bevy::app::FixedMainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
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
        (*position - *other_position).length_squared()
            <= (self.radius + other.radius).powi(2)
    }
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ColliderSchedule;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        let collider_schedule = Schedule::new(ColliderSchedule);
        app.add_schedule(collider_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(FixedPostUpdate, ColliderSchedule);
    }
}

#[cfg(test)]
mod collider_tests {
    use super::*;

    #[test]
    fn test_collides() {
        assert!(CircleCollider { radius: 1. }.collides(
            &Position::new(Vec2::new(-1024., -1024.)),
            &CircleCollider { radius: 1. },
            &Position::new(Vec2::new(1023., 1023.))
        ));
        assert!(!CircleCollider { radius: 1. }.collides(
            &Position::new(Vec2::new(-1023., -1023.)),
            &CircleCollider { radius: 1. },
            &Position::new(Vec2::new(1023., 1023.))
        ));
    }
}
