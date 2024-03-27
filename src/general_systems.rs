use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::game_components::{collider::*, components::*, wrap::*};
use crate::player::*;
use crate::asteroid::*;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 1024.,
                max_height: 1024.,
            },
            near: -1000.0,
            far: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });

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
        Drag(0.95),
        Collider(CircleCollider { radius: 4.0 }),
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            texture: asset_server.load("spaceship.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
    ));
    commands.spawn((
        Velocity(Vec2::ZERO),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(4096.0, 4096.0)),
                ..Default::default()
            },
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0., 0., 0.0),
            ..Default::default()
        },
    ));
    commands.spawn((
        Asteroid::Attached,
        Velocity(Vec2::ZERO),
        Drag(0.95),
        Collider(CircleCollider { radius: 12.0 }),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            texture: asset_server.load("asteroid.png"),
            transform: Transform::from_xyz(0., 0., 1.),
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
        transform.translation +=
            ((velocity.0 - player_velocity.0) * time.delta_seconds()).extend(0.0);
        transform.translation = wrap(transform.translation);
    }
}

pub fn apply_drags(mut query: Query<(&mut Velocity, &Drag)>, time: Res<Time>) {
    for (mut velocity, drag) in query.iter_mut() {
        velocity.0 *= f32::powf(1. - drag.0, time.delta_seconds());
    }
}
