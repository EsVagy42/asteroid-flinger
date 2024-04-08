use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

use crate::game::{collider::CircleCollider, components::*};

const EXPLOSION_FRAMES: usize = 2;
const EXPLOSION_DELAY: f32 = 0.5;

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub game_components: GameComponentsBundle,
    pub collider: CircleCollider,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animator: crate::sprite_updater::animator::Animator,
}

#[derive(Event)]
pub struct ExplosionEvent(pub Entity);

fn explode(
    mut commands: Commands,
    mut event_reader: EventReader<ExplosionEvent>,
    query: Query<(&Position, &Velocity)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for event in event_reader.read() {
        let (position, velocity) = query
            .get(event.0)
            .expect("Entity does not have Position or Velocity components");
        commands.spawn(ExplosionBundle {
            game_components: GameComponentsBundle {
                position: Position(position.0),
                velocity: Velocity(velocity.0),
                ..Default::default()
            },
            collider: CircleCollider { radius: 8. },
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..Default::default()
                },
                texture: asset_server.load("explosion.png"),
                atlas: TextureAtlas {
                    layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::new(8., 8.),
                        EXPLOSION_FRAMES,
                        1,
                        None,
                        None,
                    )),
                    index: 0,
                },
                ..Default::default()
            },
            animator: crate::sprite_updater::animator::Animator::new(
                0,
                EXPLOSION_FRAMES - 1,
                EXPLOSION_DELAY,
                false,
            ),
        });
    }
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ExplosionSchedule;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        let mut explosion_schedule = Schedule::new(ExplosionSchedule);
        explosion_schedule.add_systems(explode);
        app.add_schedule(explosion_schedule);

        app.world
            .resource_mut::<FixedMainScheduleOrder>()
            .insert_after(FixedUpdate, ExplosionSchedule);

        app.add_event::<ExplosionEvent>()
            .add_systems(FixedUpdate, explode);
    }
}
