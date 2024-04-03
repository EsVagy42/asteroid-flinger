use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use crate::{game::components::{Position, Velocity}, input::JustReleasingEvent, player};

const ASTEROID_GRAVITY_MULTIPLIER: f32 = 0.0390625;
const ASTEROID_REPULSION_MULTIPLIER: f32 = 1.09375;
const INACTIVATION_SPEED_SQRD: f32 = 1.;
pub const ASTEROID_DRAG: f32 = 0.0457297;
const DETACHED_ASTEROID_DRAG: f32 = 0.010772;
const ASTEROID_PICKUP_DISTANCE_SQRD: f32 = 32. * 32.;
const ASTEROID_DETACHMENT_BOOST_MULTIPLIER: f32 = 1.5;

#[derive(Component)]
pub struct Asteroid;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Reflect, Default)]
pub enum AsteroidState {
    #[default]
    Attached,
    Flying,
    Inactive,
}

fn asteroid_becoming_detached(
    mut asteroid_state: ResMut<NextState<AsteroidState>>,
) {
    asteroid_state.set(AsteroidState::Flying);
}

fn asteroid_becoming_inactive(
    asteroid_query: Query<&Velocity, With<Asteroid>>,
    mut state: ResMut<NextState<AsteroidState>>,
) {
    let velocity = asteroid_query.single();
    if velocity.0.length_squared() < INACTIVATION_SPEED_SQRD {
        state.set(AsteroidState::Inactive);
    }
}

fn asteroid_becoming_attached(
    asteroid_query: Query<&Position, With<Asteroid>>,
    player_query: Query<&Position, With<player::Player>>,
    mut state: ResMut<NextState<AsteroidState>>,
) {
    let position = asteroid_query.single();
    let player_position = player_query.single();
    if position.0.distance_squared(player_position.0) < ASTEROID_PICKUP_DISTANCE_SQRD {
        state.set(AsteroidState::Attached);
    }
}

fn on_asteroid_attached(
    mut commands: Commands,
    asteroid_query: Query<Entity, With<Asteroid>>,
) {
    let asteroid = asteroid_query.single();
    commands.entity(asteroid).insert(
        crate::movement::asteroid_movement::AsteroidMovement {
            gravity_multiplier: ASTEROID_GRAVITY_MULTIPLIER,
            repulsion_multiplier: ASTEROID_REPULSION_MULTIPLIER,
        }
    );
}

fn on_asteroid_detached(
    mut commands: Commands,
    asteroid_query: Query<Entity, With<Asteroid>>,
) {
    let asteroid = asteroid_query.single();
    commands.entity(asteroid).remove::<crate::movement::asteroid_movement::AsteroidMovement>();
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct AsteroidSchedule;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AsteroidState::Attached);

        let mut asteroid_schedule = Schedule::new(AsteroidSchedule);
        asteroid_schedule.add_systems(
            (
                asteroid_becoming_attached.run_if(|state: Res<State<AsteroidState>>| *state != AsteroidState::Attached),
                asteroid_becoming_detached.run_if(|state: Res<State<AsteroidState>>| *state == AsteroidState::Attached).run_if(on_event::<JustReleasingEvent>()),
                asteroid_becoming_inactive.run_if(|state: Res<State<AsteroidState>>| *state == AsteroidState::Flying),
            )
        );
        app.add_schedule(asteroid_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(FixedUpdate, AsteroidSchedule);
        
        app.add_systems(OnEnter(AsteroidState::Attached), on_asteroid_attached);
        app.add_systems(OnEnter(AsteroidState::Flying), on_asteroid_detached);
        
    }
}