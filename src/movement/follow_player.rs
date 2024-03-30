use bevy::prelude::*;
use crate::game_components::components::*;
use crate::player::*;

#[derive(Component)]
pub struct FollowPlayer {
    pub speed: f32,
}

pub fn apply(
    mut query: Query<(&FollowPlayer, &Transform, &mut Acceleration)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    for (approach_player, transform, mut acceleration) in query.iter_mut() {
        acceleration.0 = ((player_transform.translation - transform.translation).normalize_or_zero() * approach_player.speed).truncate();
    }
}
