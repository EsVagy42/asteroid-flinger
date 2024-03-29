use bevy::prelude::*;

mod asteroid;
mod game_components;
mod general_systems;
mod player;

use game_components::input::*;
use general_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameInput::default())
        .add_systems(Startup, startup)
        .add_systems(Update, player::update_player_sprite)
        .add_systems(
            FixedUpdate,
            (
                update_input,
                player::update_player,
                apply_velocities,
                apply_drags,
                asteroid::update_asteroid_velocity,
                asteroid::update_asteroid_state,
            ),
        )
        .run();
}
