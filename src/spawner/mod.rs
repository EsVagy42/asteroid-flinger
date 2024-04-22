use std::collections::VecDeque;

use bevy::prelude::*;

mod game_waves;

#[derive(Clone)]
struct Spawner {
    spawner: &'static fn(),
    timer: Timer,
}

fn nofn() {}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            spawner: &(nofn as fn()),
            timer: Timer::default(),
        }
    }
}

#[derive(Resource, Clone, Default)]
struct CurrentSpawner(Spawner);

#[derive(Event)]
struct SpawnEvent;

#[derive(Clone, Default)]
struct Wave {
    spawners: VecDeque<Spawner>,
    timer: Timer,
}

#[derive(Resource, Clone, Default)]
struct CurrentWave(Wave);

#[derive(Event)]
pub struct WaveEvent;

#[derive(Clone, Default)]
struct GameWaves {
    waves: VecDeque<Wave>,
}

#[derive(Resource, Default)]
struct CurrentGameWaves(GameWaves);

fn update_current_spawner(
    mut spawner: ResMut<CurrentSpawner>,
    mut event_writer: EventWriter<SpawnEvent>,
    time: Res<Time>,
) {
    if spawner.0.timer.tick(time.delta()).just_finished() {
        event_writer.send(SpawnEvent);
    }
}

fn update_current_wave(
    mut wave: ResMut<CurrentWave>,
    mut event_writer: EventWriter<WaveEvent>,
    time: Res<Time>,
) {
    if wave.0.timer.tick(time.delta()).just_finished() {
        event_writer.send(WaveEvent);
    }
}

fn on_spawner_event(mut spawner: ResMut<CurrentSpawner>, mut wave: ResMut<CurrentWave>) {
    (spawner.0.spawner)();
    if let Some(next_spawner) = wave.0.spawners.front() {
        spawner.0 = next_spawner.clone();
        wave.0.spawners.pop_front();
    }
}

fn on_wave_event(mut wave: ResMut<CurrentWave>, mut game_waves: ResMut<CurrentGameWaves>) {
    if let Some(next_wave) = game_waves.0.waves.front() {
        wave.0 = next_wave.clone();
        game_waves.0.waves.pop_front();
    }
}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>();
        app.add_event::<WaveEvent>();
        app.init_resource::<CurrentGameWaves>();
        app.init_resource::<CurrentWave>();
        app.init_resource::<CurrentSpawner>();
        app.add_systems(Startup, game_waves::init_wave_resources);
        app.add_systems(
            FixedUpdate,
            (
                update_current_spawner,
                update_current_wave,
                on_spawner_event.run_if(on_event::<SpawnEvent>()),
                on_wave_event.run_if(on_event::<WaveEvent>()),
            ),
        );
    }
}
