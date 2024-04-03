use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

#[derive(Component)]
pub struct Position(pub Vec2);

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
        game_components_schedule
            .add_systems(
                (
                   apply_acceleration,
                   apply_drag,
                   apply_velocity,
                   apply_position,
                ).chain());
        app.add_schedule(game_components_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(FixedUpdate, GameComponentsSchedule);
    }
}