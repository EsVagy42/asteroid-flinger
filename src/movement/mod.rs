use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

pub mod input_movement;

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct MovementSchedule;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        let movement_schedule = Schedule::new(MovementSchedule);
        app.add_schedule(movement_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(FixedUpdate, MovementSchedule);
        
        app.add_plugins((
            input_movement::InputMovementPlugin,
        ));
    }
}