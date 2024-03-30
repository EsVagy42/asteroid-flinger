use bevy::prelude::*;
use crate::game_components::components::*;

const MIN_ACCELERATION_SQRD: f32 = 1.;

#[derive(Component)]
pub struct DirectionalUpdater {
    pub offset: usize,
}

pub fn update(
    mut query: Query<(&DirectionalUpdater, &Acceleration, &mut TextureAtlas)>,
) {
    for (simple_updater, acceleration, mut texture_atlas) in query.iter_mut() {
        if acceleration.0.length_squared() > MIN_ACCELERATION_SQRD {
            texture_atlas.index = ((acceleration.0.to_angle() + std::f32::consts::PI + 0.3927) / 0.7854) as usize % 8 + simple_updater.offset;
        }
    }
}