use bevy::prelude::*;

use crate::game_components::{components::*, input::*};

const PLAYER_SPEED: f32 = 1.375;
pub const PLAYER_DRAG: f32 = 0.0457297;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerPosition(pub Vec2);

pub fn update_player(
    mut player_query: Query<(&mut Sprite, &mut TextureAtlas, &mut Velocity), With<Player>>,
    input: Res<GameInput>,
) {
    let (mut sprite, mut texture_atlas, mut velocity) = player_query.single_mut();
    input.sprite_modifier.apply_to_sprite(sprite.as_mut());
    input.sprite_modifier.apply_to_atlas(texture_atlas.as_mut());
    velocity.0 += input.direction * PLAYER_SPEED;
}