use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::asteroid::*;
use crate::enemies::Enemy;
use crate::game_components::{collider::*, components::*, wrap::*};
use crate::player::*;
use crate::sprite_updater::*;

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
        8,
        1,
        None,
        None,
    ));
    commands.spawn((
        Player,
        directional_updater::DirectionalUpdater {
            offset: 0,
        },
        Velocity(Vec2::ZERO),
        Acceleration(Vec2::ZERO),
        Drag(crate::player::PLAYER_DRAG),
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
        Acceleration(Vec2::ZERO),
        Drag(crate::asteroid::ASTEROID_DRAG),
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
    commands.spawn((
        Enemy,
        crate::movement::approach_player::ApproachPlayer {
            speed: 0.0005,
        },
        Collider(CircleCollider { radius: 8.0 }),
        Velocity(Vec2::ZERO),
        Acceleration(Vec2::ZERO),
        Drag(crate::asteroid::ASTEROID_DRAG),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            texture: asset_server.load("sample_enemy.png"),
            transform: Transform::from_xyz(512.0, 512.0, 1.0),
            ..Default::default()
        }
    ));
}

pub fn apply_acceleration(
    mut query: Query<(&mut Velocity, &Acceleration)>,
) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0;
    }
}

pub fn apply_velocities(
    mut query: Query<(&mut Transform, &Velocity), Without<Player>>,
    player: Query<&Velocity, With<Player>>,
) {
    let player_velocity = player.single();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation +=
            (velocity.0 - player_velocity.0).extend(0.0);
        transform.translation = wrap(transform.translation);
    }
}

pub fn apply_drags(mut query: Query<(&mut Velocity, &Drag)>) {
    for (mut velocity, drag) in query.iter_mut() {
        velocity.0 *= 1. - drag.0;
    }
}

pub fn handle_asteroid_enemy_collision (
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Collider), With<Enemy>>,
    asteroid: Query<(&Transform, &Collider), With<Asteroid>>,
) {
    let (asteroid_transform, asteroid_collider) = asteroid.single();
    for (entity, enemy_transform, enemy_collider) in enemy_query.iter() {
        if check_collision(asteroid_transform, asteroid_collider, enemy_transform, enemy_collider) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_player_enemy_collision (
    mut commands: Commands,
    enemy_query: Query<(&Transform, &Collider), With<Enemy>>,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
) {
    let (player, player_transform, player_collider) = player_query.single();
    for (enemy_transform, enemy_collider) in enemy_query.iter() {
        if check_collision(player_transform, player_collider, enemy_transform, enemy_collider) {
            commands.entity(player).despawn();
        }
    }
}