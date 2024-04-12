use std::ops::*;
use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use crate::game::wrap::wrap_vec2;

#[derive(Component, Clone, Copy)]
pub struct Position(Vec2);

impl Add<Vec2> for Position {
    type Output = Position;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self(wrap_vec2(self.0 + rhs))
    }
}

impl Sub<Position> for Position {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        wrap_vec2(self.0 - rhs.0)
    }
}

impl Sub<Vec2> for Position {
    type Output = Position;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self(wrap_vec2(self.0 - rhs))
    }
}

impl Position {
    pub fn new(vec: Vec2) -> Self {
        Self(wrap_vec2(vec))
    }
    
    pub fn get(&self) -> Vec2 {
        self.0
    }
    
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.0.extend(0.))
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct Drag(pub f32);

fn apply_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0;
    }
}

fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0;
    }
}

fn apply_drag(mut query: Query<(&mut Velocity, &Drag)>) {
    for (mut velocity, drag) in query.iter_mut() {
        velocity.0 *= 1.0 - drag.0;
    }
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct GameComponentsSchedule;

pub struct GameComponentsPlugin;

impl Plugin for GameComponentsPlugin {
    fn build(&self, app: &mut App) {
        let mut game_components_schedule = Schedule::new(GameComponentsSchedule);
        game_components_schedule.add_systems(
            (
                apply_position,
                apply_acceleration,
                apply_drag,
                apply_velocity,
            )
                .chain(),
        );
        app.add_schedule(game_components_schedule);
        app.world
            .resource_mut::<FixedMainScheduleOrder>()
            .insert_after(FixedUpdate, GameComponentsSchedule);
    }
}

#[derive(Bundle)]
pub struct GameComponentsBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub drag: Drag,
}

impl GameComponentsBundle {
    pub fn new(position: Vec2, drag: f32) -> Self {
        Self {
            position: Position(position),
            velocity: Velocity(Vec2::ZERO),
            acceleration: Acceleration(Vec2::ZERO),
            drag: Drag(drag),
        }
    }
}

impl Default for GameComponentsBundle {
    fn default() -> Self {
        Self {
            position: Position(Vec2::ZERO),
            velocity: Velocity(Vec2::ZERO),
            acceleration: Acceleration(Vec2::ZERO),
            drag: Drag(0.),
        }
    }
}
