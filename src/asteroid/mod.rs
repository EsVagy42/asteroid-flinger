use std::borrow::BorrowMut;

use bevy::prelude::*;
use crate::{game::components::{Acceleration, Position, Velocity}, input::JustReleasingEvent, player};

const ASTEROID_GRAVITY_MULTIPLIER: f32 = 0.0390625;
const ASTEROID_REPULSION_MULTIPLIER: f32 = 1.09375;
const INACTIVATION_SPEED_SQRD: f32 = 1.;
pub const ASTEROID_DRAG: f32 = 0.0457297;
const DETACHED_ASTEROID_DRAG: f32 = 0.010772;
const ASTEROID_PICKUP_DISTANCE_SQRD: f32 = 32. * 32.;
const ASTEROID_DETACHMENT_BOOST_MULTIPLIER: f32 = 1.5;

#[derive(Component)]
pub struct Asteroid;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum AsteroidState {
    Attached,
    Flying,
    Inactive,
}

fn update_attached_asteroid_acceleration(
    mut asteroid_query: Query<(&Position, &mut Acceleration), With<Asteroid>>,
    player_query: Query<&Position, With<player::Player>>,   
) {
    let player_position = player_query.single();
    let (position, mut acceleration) = asteroid_query.single_mut();
    let direction = player_position.0 - position.0;
    acceleration.0 = direction * ASTEROID_GRAVITY_MULTIPLIER;
    let direction = direction.normalize_or_zero();
    acceleration.0 -= direction * ASTEROID_REPULSION_MULTIPLIER;
}

fn asteroid_becoming_detached(
    input_event: EventReader<JustReleasingEvent>,
    mut state: ResMut<State<AsteroidState>>,
) {
    todo!();
}

fn asteroid_becoming_inactive(
    asteroid_query: Query<&Velocity, With<Asteroid>>,
    mut state: ResMut<State<AsteroidState>>,
) {
    let velocity = asteroid_query.single();
    if velocity.0.length_squared() < INACTIVATION_SPEED_SQRD {
        todo!();
    }
}

fn asteroid_becoming_attached(
    asteroid_query: Query<&Position, With<Asteroid>>,
    player_query: Query<&Position, With<player::Player>>,
    mut state: ResMut<State<AsteroidState>>,
) {
    let position = asteroid_query.single();
    let player_position = player_query.single();
    if position.0.distance_squared(player_position.0) < ASTEROID_PICKUP_DISTANCE_SQRD {
        todo!();
    }
}