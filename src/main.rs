use bevy::prelude::*;

mod game;
mod input;
mod sprite_updater;
mod movement;
mod player;
mod asteroid;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::GamePlugin, input::InputPlugin, sprite_updater::SpriteUpdaterPlugin, movement::MovementPlugin, player::PlayerPlugin))
        .run();
}