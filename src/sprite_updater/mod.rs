use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

pub mod directional_updater;
pub mod animator;

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct SpriteUpdaterSchedule;

pub struct SpriteUpdaterPlugin;

impl Plugin for SpriteUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            directional_updater::DirectionalUpdaterPlugin,
            animator::AnimatorPlugin,
        ));
    }
}