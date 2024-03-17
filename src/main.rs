use bevy::prelude::*;

mod mod_float;
use mod_float::*;
mod mod_float_vec;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

