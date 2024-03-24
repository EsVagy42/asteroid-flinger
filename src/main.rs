use asteroid::update_asteroid_velocity;
use bevy::prelude::*;

mod asteroid;
mod general_components;
mod general_systems;
mod input;
mod player;
mod wrap;

use crate::general_components::*;
use crate::general_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(input::GameInput::default())
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                input::update_input,
                player::update_player,
                apply_velocities,
                apply_drags,
                update_asteroid_velocity,
            ),
        )
        .run();
}
