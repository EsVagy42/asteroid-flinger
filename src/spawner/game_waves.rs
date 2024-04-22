use bevy::prelude::*;

use super::*;

pub fn init_wave_resources(
    mut current_game_waves: ResMut<CurrentGameWaves>,
    mut current_waves: ResMut<CurrentWave>,
    mut current_spawner: ResMut<CurrentSpawner>,
) {
    let game_waves = GameWaves {
        waves: vec![Wave {
            spawners: vec![Spawner::default()].into(),
            timer: Timer::from_seconds(30., TimerMode::Once),
        }]
        .into(),
    };
    current_game_waves.0 = game_waves.clone();
    current_waves.0 = game_waves.waves[0].clone();
    current_spawner.0 = game_waves.waves[0].spawners[0].clone();
}
