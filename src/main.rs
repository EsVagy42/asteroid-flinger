use bevy::prelude::*;

mod wrap;
mod player;
mod general_components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

