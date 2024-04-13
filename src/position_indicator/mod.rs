use crate::game::components::*;
use bevy::prelude::*;

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
    mut query: Query<(&PositionIndicator, &CircleIndicator, &mut Transform)>,
    parent_query: Query<&Position>,
) {
    for (position_indicator, circle_indicator, mut transform) in
        query.iter_mut()
    {
        if let Ok(parent_position) = parent_query.get(position_indicator.0) {
            *transform = Transform::from_translation(
                (parent_position.get().normalize() * circle_indicator.radius).extend(1.),
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

fn update_offscreen_indicator(
    mut query: Query<(
        &PositionIndicator,
        &mut Transform,
        &mut Visibility,
        &Sprite,
    ), With<OffscreenIndicator>>,
    mut parent_query: Query<&Position>,
    mut camera_query: Query<&Camera>,
) {
    let camera = camera_query.single_mut();
    for (
        position_indicator,
        mut transform,
        mut visibility,
        sprite,
    ) in query.iter_mut()
    {
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
                *transform = Transform::from_translation(indicator_position_clamped.extend(1.));
            }
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
        app.add_systems(
            Update,
            (
                despawn_indicator,
                update_circle_indicator,
                update_offscreen_indicator,
                update_indicator_atlas,
            ),
        );
    }
}
