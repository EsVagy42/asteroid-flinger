use bevy::{prelude::*, render::camera::ScalingMode};

use crate::game::collider::*;
use crate::game::components::*;

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

    commands.spawn((
        crate::player::Player,
        crate::movement::input_movement::InputMovement {
            speed: crate::player::PLAYER_ACCELERATION,
        },
        crate::sprite_updater::directional_updater::DirectionalUpdater { offset: 0 },
        Position(Vec2::ZERO),
        Velocity(Vec2::ZERO),
        Acceleration(Vec2::ZERO),
        Drag(crate::player::PLAYER_DRAG),
        CircleCollider { radius: 4.0 },
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            texture: asset_server.load("spaceship.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(8., 8.),
                    13,
                    1,
                    None,
                    None,
                )),
                index: 0,
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
    ));
    commands.spawn((
        Position(Vec2::ZERO),
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
        crate::asteroid::Asteroid,
        Position(Vec2::ZERO),
        Velocity(Vec2::ZERO),
        Acceleration(Vec2::ZERO),
        Drag(crate::asteroid::ASTEROID_DRAG),
        CircleCollider { radius: 12.0 },
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
        crate::enemy::Enemy,
        Position(Vec2::new(256., 256.)),
        Velocity(Vec2::ZERO),
        Acceleration(Vec2::ZERO),
        Drag(crate::asteroid::ASTEROID_DRAG),
        CircleCollider { radius: 8.0 },
        crate::movement::follow_player::FollowPlayer { speed: 0.05 },
        crate::sprite_updater::directional_updater::DirectionalUpdater { offset: 0 },
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            texture: asset_server.load("spaceship.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(8., 8.),
                    8,
                    1,
                    None,
                    None,
                )),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
    ));
}
