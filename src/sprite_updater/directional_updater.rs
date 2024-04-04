use bevy::prelude::*;
use crate::game::components::Acceleration;

const MIN_ACCELERATION_SQRD: f32 = 0.0001;

#[derive(Component)]
pub struct DirectionalUpdater {
    pub offset: usize,
}

pub fn update(
    mut query: Query<(&DirectionalUpdater, &Acceleration, &mut TextureAtlas)>,
) {
    for (simple_updater, acceleration, mut texture_atlas) in query.iter_mut() {
        if acceleration.0.length_squared() > MIN_ACCELERATION_SQRD {
            texture_atlas.index = ((acceleration.0.to_angle() + std::f32::consts::PI + (std::f32::consts::PI / 8.)) / (std::f32::consts::PI / 4.)) as usize % 8 + simple_updater.offset;
        }
    }
}

pub struct DirectionalUpdaterPlugin;

impl Plugin for DirectionalUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update);
    }
}