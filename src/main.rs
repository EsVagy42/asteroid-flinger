use bevy::prelude::*;

mod general_components;
mod general_systems;
mod player;
mod wrap;
mod input;

use crate::general_components::*;
use crate::general_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, startup)
        .add_systems(Update, (input::update_input, player::update_player).chain())
        .insert_resource(player::PlayerPosition(Vec2::ZERO))
        .insert_resource(input::GameInput::default())
        .run();
}
