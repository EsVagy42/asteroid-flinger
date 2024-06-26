use crate::asteroid::Asteroid;
use crate::explosion::ExplosionEvent;
use crate::explosion::*;
use crate::game::components::{GameComponentsBundle, Position, Velocity};
use bevy::app::FixedMainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

use crate::game::collider::CircleCollider;

#[derive(Component)]
pub struct Enemy;

#[derive(Event)]
pub struct EnemyDespawnEvent(pub Entity);

fn check_for_destructive_collision(
    mut enemy_query: Query<(Entity, &CircleCollider, &Position, &mut Velocity), With<Enemy>>,
    query: Query<
        (&CircleCollider, &Position, &Velocity),
        (Or<(With<Asteroid>, With<Explosion>)>, Without<Enemy>),
    >,
    mut explosion_event_writer: EventWriter<ExplosionEvent>,
    mut despawn_event_writer: EventWriter<EnemyDespawnEvent>,
) {
    'enemy_loop: for (entity, collider, position, mut velocity) in enemy_query.iter_mut() {
        for (other_collider, other_position, other_velocity) in query.iter() {
            if collider.collides(position, other_collider, other_position) {
                let new_velocity = (*position - (*other_position - other_velocity.0))
                    .normalize_or_zero()
                    * other_velocity.0.length();
                velocity.0 = new_velocity;

                explosion_event_writer.send(ExplosionEvent(entity));
                despawn_event_writer.send(EnemyDespawnEvent(entity));

                continue 'enemy_loop;
            }
        }
    }
}

fn despawn_enemy(mut commands: Commands, mut despawn_event_reader: EventReader<EnemyDespawnEvent>) {
    for event in despawn_event_reader.read() {
        commands.entity(event.0).despawn();
    }
}

fn despawn_every_enemy(query: Query<Entity, With<Enemy>>, mut commands: Commands) {
    for enemy in query.iter() {
        commands.entity(enemy).despawn();
    }
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct EnemyDespawnSchedule;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDespawnEvent>();

        let mut enemy_despawn_schedule = Schedule::new(EnemyDespawnSchedule);
        enemy_despawn_schedule.add_systems((
            despawn_enemy.run_if(on_event::<EnemyDespawnEvent>()),
            despawn_every_enemy.run_if(on_event::<crate::spawner::WaveEvent>()),
        ));
        app.add_schedule(enemy_despawn_schedule);
        app.world
            .resource_mut::<FixedMainScheduleOrder>()
            .insert_after(crate::explosion::ExplosionSchedule, EnemyDespawnSchedule);

        app.add_systems(
            crate::game::collider::ColliderSchedule,
            check_for_destructive_collision,
        );
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub game_components: GameComponentsBundle,
    pub collider: CircleCollider,
    pub sprite_sheet_bundle: SpriteSheetBundle,
}
