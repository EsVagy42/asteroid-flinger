use bevy::prelude::*;

use crate::general_components::*;
use crate::player::Player;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(8., 8.),
        3,
        1,
        None,
        None,
    ));
    commands.spawn((
        Player,
        Velocity(Vec2::ZERO),
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            texture: asset_server.load("spaceship.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            ..Default::default()
        },
    ));
}

pub fn apply_velocities(
    mut query: Query<(&mut Transform, &Velocity), Without<Player>>,
    player: Query<&Velocity, With<Player>>,
    time: Res<Time>,
) {
    let player_velocity = player.single();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += ((velocity.0 - player_velocity.0) * time.delta_seconds()).extend(1.0);
    } 
} 

pub fn apply_drags(
    mut query: Query<(&mut Velocity, &Drag)>,
    time: Res<Time>,
) {
    for (mut velocity, drag) in query.iter_mut() {
        velocity.0 *= f32::powf(1. - drag.0, time.delta_seconds());
    }
}