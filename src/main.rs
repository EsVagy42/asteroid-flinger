use bevy::prelude::*;

mod general_components;
mod general_systems;
mod player;
mod wrap;

use crate::general_components::*;
use crate::general_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}
