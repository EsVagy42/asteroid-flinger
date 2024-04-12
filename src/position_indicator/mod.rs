use crate::game::components::*;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct CircleIndicator {
    pub entity: Entity,
    pub radius: f32,
}

fn update_circle_indicator(
    mut query: Query<(&CircleIndicator, &mut Transform)>,
    parent_query: Query<&Position>,
) {
    for (circle_indicator, mut transform) in query.iter_mut() {
        let parent_position = parent_query
            .get(circle_indicator.entity)
            .expect("CircleIndicator must have a parent");
        *transform = Transform::from_translation(
            (parent_position.get().normalize() * circle_indicator.radius).extend(1.),
        );
    }
}

#[derive(Component)]
pub struct OffscreenIndicator {
    pub entity: Entity,
}

fn update_offscreen_indicator(
    mut query: Query<(
        &OffscreenIndicator,
        &mut Transform,
        &mut Visibility,
        &Sprite,
    )>,
    mut parent_query: Query<&Position>,
    mut camera_query: Query<&Camera>,
) {
    let camera = camera_query.single_mut();
    for (offscreen_indicator, mut transform, mut visibility, sprite) in query.iter_mut() {
        let position = parent_query
            .get_mut(offscreen_indicator.entity)
            .expect("OffscreenIndicator must have a parent");
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

pub struct PositionIndicatorPlugin;

impl Plugin for PositionIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_circle_indicator, update_offscreen_indicator),
        );
    }
}
