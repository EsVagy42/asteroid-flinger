use bevy::prelude::*;
use crate::game::components::{GameComponentsBundle, Position};
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

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub game_components: GameComponentsBundle,
    pub collider: CircleCollider,
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

impl EnemyBundle {
    pub fn new(position: Vec2, drag: f32, collider_radius: f32, image: Handle<Image>, sprite_size: Vec2, texture_atlas: TextureAtlas) -> Self {
        Self {
            enemy: Enemy,
            game_components: GameComponentsBundle::new(position, drag),
            collider: CircleCollider { radius: collider_radius },
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                texture: image,
                atlas: texture_atlas,
                transform: Transform::from_xyz(0., 0., 1.),
                ..Default::default()
            },
        }        
    }
}