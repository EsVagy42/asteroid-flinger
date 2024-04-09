use bevy::prelude::*;

pub mod components;
pub mod collider;
pub mod wrap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                components::GameComponentsPlugin,
                wrap::WrapPlugin,
                collider::ColliderPlugin,
            ));
    }
}