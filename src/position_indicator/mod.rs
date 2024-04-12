use crate::game::components::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct IndicatorDespawnEvent(pub Entity);

fn despawn_indicator(
    mut commands: Commands,
    mut event_reader: EventReader<IndicatorDespawnEvent>,
    query: Query<Entity>,
) {
    for event in event_reader.read() {
        if query.get(event.0).is_ok() {
            commands.entity(event.0).despawn();
        }
    }
}

#[derive(Component)]
pub struct CircleIndicator {
    pub entity: Entity,
    pub radius: f32,
}

fn update_circle_indicator(
    mut query: Query<(Entity, &CircleIndicator, &mut Transform)>,
    parent_query: Query<&Position>,
    mut despawn_writer: EventWriter<IndicatorDespawnEvent>,
) {
    for (circle_indicator_entity, circle_indicator, mut transform) in query.iter_mut() {
        if let Ok(parent_position) = parent_query.get(circle_indicator.entity) {
            *transform = Transform::from_translation(
                (parent_position.get().normalize() * circle_indicator.radius).extend(1.),
            );
        } else {
            despawn_writer.send(IndicatorDespawnEvent(circle_indicator_entity));
        }
    }
}

#[derive(Component)]
pub struct OffscreenIndicator {
    pub entity: Entity,
}

fn update_offscreen_indicator(
    mut query: Query<(
        Entity,
        &OffscreenIndicator,
        &mut Transform,
        &mut Visibility,
        &Sprite,
    )>,
    mut parent_query: Query<&Position>,
    mut camera_query: Query<&Camera>,
    mut despawn_writer: EventWriter<IndicatorDespawnEvent>,
) {
    let camera = camera_query.single_mut();
    for (offscreen_indicator_entity, offscreen_indicator, mut transform, mut visibility, sprite) in
        query.iter_mut()
    {
        if let Ok(position) = parent_query.get_mut(offscreen_indicator.entity) {
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
                *transform = Transform::from_translation(indicator_position_clamped.extend(1.));
            }
        } else {
            despawn_writer.send(IndicatorDespawnEvent(offscreen_indicator_entity));
        }
    }
}

pub struct PositionIndicatorPlugin;

impl Plugin for PositionIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IndicatorDespawnEvent>();
        app.add_systems(
            Update,
            (
                despawn_indicator,
                update_circle_indicator,
                update_offscreen_indicator,
            ),
        );
    }
}
