use bevy::prelude::*;
use crate::game::components::Position;
use crate::asteroid::Asteroid;

use crate::game::collider::CircleCollider;

#[derive(Component)]
pub struct Enemy;

fn check_asteroid_collision(
    mut commands: Commands,
    enemy_query: Query<(Entity, &CircleCollider, &Position), With<Enemy>>,
    asteroid_query: Query<(&CircleCollider, &Position), With<Asteroid>>,
) {
    let (asteroid_collider, asteroid_position) = asteroid_query.single();
    for (entity, collider, position) in enemy_query.iter() {
        if collider.collides(position, asteroid_collider, asteroid_position) {
            commands.entity(entity).despawn();
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(crate::game::collider::ColliderSchedule, check_asteroid_collision);
    }
}