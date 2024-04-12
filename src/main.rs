use bevy::prelude::*;

mod asteroid;
mod game;
mod input;
mod movement;
mod player;
mod sprite_updater;
mod startup;
mod enemy;
mod explosion;
mod position_indicator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            game::GamePlugin,
            input::InputPlugin,
            sprite_updater::SpriteUpdaterPlugin,
            movement::MovementPlugin,
            explosion::ExplosionPlugin,
            position_indicator::PositionIndicatorPlugin,
            player::PlayerPlugin,
            asteroid::AsteroidPlugin,
            enemy::EnemyPlugin,
        ))
        .add_systems(Startup, startup::startup)
        .run();
}

