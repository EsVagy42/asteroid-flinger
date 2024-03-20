use bevy::prelude::*;

use crate::player::Player;
use crate::general_components::*;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(8., 8.), 3, 1, None, None));
    commands.spawn((
        Player,
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
