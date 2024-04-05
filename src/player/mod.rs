use std::default;

use crate::game::{collider::CircleCollider, components::Position};
use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

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

fn check_enemy_collision(
    player_query: Query<(&CircleCollider, &Position), With<Player>>,
    enemy_query: Query<(&CircleCollider, &Position), With<crate::enemy::Enemy>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    let (player_collider, player_position) = player_query.single();
    for (enemy_collider, enemy_position) in enemy_query.iter() {
        if player_collider.collides(player_position, enemy_collider, enemy_position) {
            next_state.set(PlayerState::Dead);
        }
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Reflect, Default)]
enum PlayerState {
    #[default]
    Alive,
    Dead,
    Invincible,
}

#[derive(Resource, Default)]
struct PlayerStateTimer(Timer);

fn set_player_state_timer(next_state: Res<State<PlayerState>>, mut timer: ResMut<PlayerStateTimer>) {
    timer.0 = match next_state.get() {
        PlayerState::Dead => Timer::from_seconds(2., TimerMode::Once),
        PlayerState::Invincible => Timer::from_seconds(3., TimerMode::Once),
        _ => panic!("Invalid player state"),
    }
}

fn update_player_state(
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>,
    mut timer: ResMut<PlayerStateTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(match state.get() {
            PlayerState::Dead => PlayerState::Invincible,
            PlayerState::Invincible => PlayerState::Alive,
            _ => panic!("Invalid player state"),
        })
    }
}

fn on_player_dead(mut commands: Commands, mut player_query: Query<(Entity, &mut crate::game::components::Acceleration), With<Player>>) {
    let (player, mut acceleration) = player_query.single_mut();
    commands
        .entity(player)
        .remove::<crate::movement::input_movement::InputMovement>();
    acceleration.0 = Vec2::ZERO;
}

fn on_player_invincible(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player = player_query.single();
    commands
        .entity(player)
        .insert(crate::movement::input_movement::InputMovement {
            speed: PLAYER_ACCELERATION,
        });
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct PlayerSchedule;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>();
        app.init_resource::<PlayerStateTimer>();

        let mut player_schedule = Schedule::new(PlayerSchedule);
        player_schedule.add_systems((
            center_player,
            update_player_state
                .run_if(|state: Res<State<PlayerState>>| *state != PlayerState::Alive),
        ));
        app.add_schedule(player_schedule);
        app.world
            .resource_mut::<FixedMainScheduleOrder>()
            .insert_after(
                crate::game::components::GameComponentsSchedule,
                PlayerSchedule,
            );
        app.add_systems(
            crate::game::collider::ColliderSchedule,
            check_enemy_collision
                .run_if(|state: Res<State<PlayerState>>| *state == PlayerState::Alive),
        );
        app.add_systems(
            OnEnter(PlayerState::Dead),
            (set_player_state_timer, on_player_dead),
        );
        app.add_systems(
            OnEnter(PlayerState::Invincible),
            (set_player_state_timer, on_player_invincible),
        );
    }
}
