use crate::game::components::Position;
use crate::game::wrap;
use bevy::app::FixedMainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
    pub handler: Option<fn(&Entity)>,
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

fn handle_collisions(
    query: Query<(Entity, &CircleCollider, &Position)>,
) {
    let entities: Vec<(Entity, &CircleCollider, &Position)> = query.iter().collect();
    for (i, (entity, collider, position)) in entities.iter().enumerate() {
        for (other_entity, other_collider, other_position) in entities.iter().skip(i + 1) {
            if collider.collides(position, other_collider, other_position) {
                if let Some(handler) = collider.handler {
                    handler(other_entity);
                }
                if let Some(handler) = other_collider.handler {
                    handler(entity);
                }
            }
        }
    }
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct CollisionSchedule;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        let mut collision_schedule = Schedule::new(CollisionSchedule);
        collision_schedule.add_systems(handle_collisions);
        app.add_schedule(collision_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(crate::game::wrap::WrapSchedule, CollisionSchedule);
    }
}

#[cfg(test)]
mod collider_tests {
    use super::*;

    #[test]
    fn test_collides() {
        assert!(CircleCollider { radius: 1., handler: None }.collides(
            &Position(Vec2::new(-1024., -1024.)),
            &CircleCollider { radius: 1., handler: None },
            &Position(Vec2::new(1023., 1023.))
        ));
        assert!(!CircleCollider { radius: 1., handler: None }.collides(
            &Position(Vec2::new(-1023., -1023.)),
            &CircleCollider { radius: 1., handler: None },
            &Position(Vec2::new(1023., 1023.))
        ));
    }
}
