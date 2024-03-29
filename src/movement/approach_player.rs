use bevy::prelude::*;
use crate::game_components::components::*;
use crate::player::*;

#[derive(Component)]
pub struct ApproachPlayer {
    pub speed: f32,
}

pub fn apply(
    mut query: Query<(&ApproachPlayer, &Transform, &mut Velocity)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    for (approach_player, transform, mut velocity) in query.iter_mut() {
        velocity.0 = ((player_transform.translation - transform.translation) * approach_player.speed).truncate();
    }
}