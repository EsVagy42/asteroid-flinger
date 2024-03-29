use bevy::prelude::*;
use crate::game_components::components::*;
use crate::player::*;

#[derive(Component)]
pub struct SimpleUpdater;

pub fn update(
    query: Query<(&Velocity, &mut TextureAtlas), With<SimpleUpdater>>,
) {
    for (velocity, mut texture_atlas) in query.iter_mut() {
        todo!();
    }
}