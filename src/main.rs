use bevy::prelude::*;

mod asteroid;
mod enemies;
mod game_components;
mod general_systems;
mod movement;
mod player;
mod sprite_updater;

use game_components::input::*;
use general_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameInput::default())
        .add_systems(Startup, startup)
        .add_systems(
            FixedUpdate,
            (
                update_input,
                player::update_player,
                apply_acceleration,
                apply_velocities,
                apply_drags,
                asteroid::update_asteroid_velocity,
                asteroid::update_asteroid_state,
                handle_asteroid_enemy_collision,
                handle_player_enemy_collision,
            ).chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                movement::approach_player::apply,
                movement::follow_player::apply,
            ),
        )
        .add_systems(Update, (sprite_updater::directional_updater::update))
        .run();
}
