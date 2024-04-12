use bevy::prelude::*;
use crate::game::components::*;

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
        let parent_position = parent_query.get(circle_indicator.entity).expect("CircleIndicator must have a parent");
        *transform = Transform::from_translation((parent_position.get().normalize() * circle_indicator.radius).extend(1.));
    }
}

pub struct PositionIndicatorPlugin;

impl Plugin for PositionIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_circle_indicator);
    }
}