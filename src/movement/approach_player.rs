use bevy::prelude::*;
use crate::game::components::{Position, Acceleration};
use crate::player::Player;

use super::MovementSchedule;

#[derive(Component)]
pub struct ApproachPlayer {
    pub speed: f32,
}

pub fn apply(
    mut query: Query<(&ApproachPlayer, &Position, &mut Acceleration)>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_position = player_query.single();
    for (approach_player, position, mut acceleration) in query.iter_mut() {
        acceleration.0 = (*player_position - *position) * approach_player.speed;
    }
}

pub struct ApproachPlayerPlugin;

impl Plugin for ApproachPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(MovementSchedule, apply);
    }
}