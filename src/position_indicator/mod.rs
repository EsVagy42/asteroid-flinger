use crate::game::components::*;
use bevy::prelude::*;

const INDICATOR_Z_POSITION: f32 = 900.;
const INDICATOR_OPACITY_DIVIDER: f32 = crate::game::wrap::MODULO_HALF;

#[derive(Component)]
pub struct PositionIndicator(pub Entity);

fn despawn_indicator(
    mut commands: Commands,
    query: Query<(Entity, &PositionIndicator)>,
    parent_query: Query<Entity, Without<PositionIndicator>>,
) {
    for (entity, position_indicator) in query.iter() {
        if let Err(_) = parent_query.get(position_indicator.0) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct CircleIndicator {
    pub radius: f32,
}

#[derive(Bundle)]
pub struct CircleIndicatorBundle {
    pub position_indicator: PositionIndicator,
    pub circle_indicator: CircleIndicator,
}

fn update_circle_indicator(
    mut query: Query<(
        &PositionIndicator,
        &CircleIndicator,
        &mut Transform,
        &Visibility,
    )>,
    parent_query: Query<&Position>,
) {
    for (position_indicator, circle_indicator, mut transform, visibility) in query.iter_mut() {
        if let Visibility::Hidden = visibility {
            continue;
        }
        if let Ok(parent_position) = parent_query.get(position_indicator.0) {
            *transform = Transform::from_translation(
                (parent_position.get().normalize() * circle_indicator.radius)
                    .extend(INDICATOR_Z_POSITION),
            );
        }
    }
}

#[derive(Component)]
pub struct OffscreenIndicator;

#[derive(Bundle)]
pub struct OffscreenIndicatorBundle {
    pub position_indicator: PositionIndicator,
    pub offscreen_indicator: OffscreenIndicator,
}

#[derive(Event)]
pub struct OffscreenIndicatorSpawnEvent {
    indicated_entity: Entity,
}

fn spawn_offscreen_indicator(
    mut commands: Commands,
    mut spawn_event_reader: EventReader<OffscreenIndicatorSpawnEvent>,
    query: Query<(&Sprite, &Handle<Image>, &TextureAtlas)>,
) {
    for spawn_event in spawn_event_reader.read() {
        let (sprite, image_handle, texture_atlas) =
            query.get(spawn_event.indicated_entity).unwrap();

        commands.spawn((
            OffscreenIndicatorBundle {
                position_indicator: PositionIndicator(spawn_event.indicated_entity),
                offscreen_indicator: OffscreenIndicator,
            },
            sprite.clone(),
            image_handle.clone(),
            texture_atlas.clone(),
        ));
    }
}

fn update_offscreen_indicator(
    mut query: Query<
        (&PositionIndicator, &mut Transform, &mut Visibility, &Sprite),
        With<OffscreenIndicator>,
    >,
    mut parent_query: Query<&Position>,
    mut camera_query: Query<&Camera>,
) {
    let camera = camera_query.single_mut();
    for (position_indicator, mut transform, mut visibility, sprite) in query.iter_mut() {
        if let Ok(position) = parent_query.get_mut(position_indicator.0) {
            let normalized_device_position = camera
                .world_to_ndc(
                    &GlobalTransform::default(),
                    position.get_transform().translation,
                )
                .unwrap();
            let normalized_positive_device_position = Vec2::new(
                normalized_device_position.x.abs(),
                normalized_device_position.y.abs(),
            );
            let scale = normalized_positive_device_position
                .x
                .max(normalized_positive_device_position.y);
            if scale < 1.0 {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
                let indicator_position = position.get() * (1. / scale);
                let camera_bounds = camera
                    .ndc_to_world(&GlobalTransform::default(), Vec3::new(1., 1., 1.))
                    .unwrap()
                    .truncate();
                let indicator_half_size = sprite
                    .custom_size
                    .expect("This function only works with sprites with custom sizes")
                    / 2.;
                let indicator_position_clamped = Vec2::new(
                    indicator_position.x.clamp(
                        -camera_bounds.x + indicator_half_size.x,
                        camera_bounds.x - indicator_half_size.x,
                    ),
                    indicator_position.y.clamp(
                        -camera_bounds.y + indicator_half_size.y,
                        camera_bounds.y - indicator_half_size.y,
                    ),
                );
                *transform = Transform::from_translation(
                    indicator_position_clamped.extend(INDICATOR_Z_POSITION),
                );
            }
        }
    }
}

fn set_indicator_opacity(
    mut query: Query<(&mut Sprite, &PositionIndicator)>,
    parent_query: Query<&Position>,
) {
    for (mut sprite, position_indicator) in query.iter_mut() {
        if let Ok(parent_position) = parent_query.get(position_indicator.0) {
            let opacity = 1.
                - (f32::max(parent_position.get().x.abs(), parent_position.get().y.abs())
                    / INDICATOR_OPACITY_DIVIDER);
            sprite.color.set_a(opacity);
        }
    }
}

fn update_indicator_atlas(
    mut query: Query<(&PositionIndicator, &mut TextureAtlas)>,
    parent_query: Query<&TextureAtlas, Without<PositionIndicator>>,
) {
    for (position_indicator, mut texture_atlas) in query.iter_mut() {
        if let Ok(parent_texture_atlas) = parent_query.get(position_indicator.0) {
            texture_atlas.index = parent_texture_atlas.index;
        }
    }
}

pub struct PositionIndicatorPlugin;

impl Plugin for PositionIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OffscreenIndicatorSpawnEvent>();
        app.add_systems(
            Update,
            (
                despawn_indicator,
                update_circle_indicator,
                update_offscreen_indicator,
                update_indicator_atlas,
                set_indicator_opacity,
                spawn_offscreen_indicator.run_if(on_event::<OffscreenIndicatorSpawnEvent>()),
            ),
        );
    }
}
