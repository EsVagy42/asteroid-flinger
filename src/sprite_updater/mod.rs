use bevy::prelude::*;

pub mod directional_updater;
pub mod animator;

pub struct SpriteUpdaterPlugin;

impl Plugin for SpriteUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            directional_updater::DirectionalUpdaterPlugin,
            animator::AnimatorPlugin,
        ));
    }
}