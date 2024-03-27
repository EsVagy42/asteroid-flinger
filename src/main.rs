use bevy::prelude::*;

mod game_components;
mod player;
mod asteroid;
mod general_systems;

use game_components::input::*;
use general_systems::*;
use asteroid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameInput::default())
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                update_input,
                player::update_player,
                apply_velocities,
                apply_drags,
                update_asteroid_velocity,
                update_asteroid_state,
            ),
        )
        .run();
}
