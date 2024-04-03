use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use crate::game::components::Position;

const MODULO: f32 = 2048.;
const MODULO_HALF: f32 = MODULO / 2.;

fn wrap_f32(x: f32) -> f32 {
    (x + MODULO_HALF).rem_euclid(MODULO) - MODULO_HALF
}

fn wrap_vec2(v: Vec2) -> Vec2 {
    Vec2::new(wrap_f32(v.x), wrap_f32(v.y))
}

fn wrap_positions(mut position: Query<&mut Position>) {
    for mut position in position.iter_mut() {
        position.0 = wrap_vec2(position.0);
    }
}

#[cfg(test)]
mod wrap_tests {
    use super::*;

    #[test]
    fn test_vec2() {
        assert_eq!(wrap_vec2(Vec2::new(-1024., -1024.) - Vec2::new(1., 1.)), Vec2::new(1023., 1023.));
    }
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct WrapSchedule;

pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn build(&self, app: &mut App) {
        let mut wrap_schedule = Schedule::new(WrapSchedule);
        wrap_schedule.add_systems(wrap_positions);
        app.add_schedule(wrap_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(crate::game::components::GameComponentsSchedule, WrapSchedule);
    }
}