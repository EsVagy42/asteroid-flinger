use bevy::prelude::*;

use crate::game_components::{components::*, input::*};

const PLAYER_SPEED: f32 = 1.375;
pub const PLAYER_DRAG: f32 = 0.0457297;

#[derive(Component)]
pub struct Player;

pub fn update_player(
    mut player_query: Query<&mut Acceleration, With<Player>>,
    input: Res<GameInput>,
) {
    let mut acceleration = player_query.single_mut();
    acceleration.0 = input.direction * PLAYER_SPEED;
}