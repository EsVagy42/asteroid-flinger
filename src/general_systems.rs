use bevy::{prelude::*, text};

use crate::player::Player;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32., 32.)),
                ..Default::default()
            },
            texture: asset_server.load("spaceship.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
    ));

}
