use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use crate::game::components::Position;

pub const PLAYER_ACCELERATION: f32 = 1.375;
pub const PLAYER_DRAG: f32 = 0.0457297;

#[derive(Component)]
pub struct Player;

fn center_player(
    mut player_query: Query<&mut Position, With<Player>>,
    mut query: Query<&mut Position, Without<Player>>,
) {
    let mut player_position = player_query.single_mut();
    for mut position in query.iter_mut() {
        position.0 -= player_position.0;
    }
    player_position.0 = Vec2::ZERO;
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct PlayerSchedule;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let mut player_schedule = Schedule::new(PlayerSchedule);
        player_schedule.add_systems(center_player);
        app.add_schedule(player_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(crate::game::components::GameComponentsSchedule, PlayerSchedule);
    }
}