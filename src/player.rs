use bevy::prelude::*;

use crate::input::GameInput;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerPosition(pub Vec2);

pub fn update_player(
    mut player_query: Query<(&mut Transform, &mut Sprite, &mut TextureAtlas), With<Player>>,
    mut player_position: ResMut<PlayerPosition>,
    input: Res<GameInput>,
) {
    let mut player = player_query.single_mut();
    player_position.as_mut().0 = player.0.translation.truncate();
    player.1.flip_x = input.sprite_modifier.flip_x;
    player.1.flip_y = input.sprite_modifier.flip_y;
    player.2.index = input.sprite_modifier.index;
}