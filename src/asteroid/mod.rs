use crate::{
    game::components::{Acceleration, Drag, Position, Velocity},
    input::JustReleasingEvent,
    player,
};
use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

const ASTEROID_GRAVITY_MULTIPLIER: f32 = 0.0390625;
const ASTEROID_REPULSION_MULTIPLIER: f32 = 1.09375;
const INACTIVATION_SPEED_SQRD: f32 = 1.;
pub const ASTEROID_DRAG: f32 = 0.0457297;
const DETACHED_ASTEROID_DRAG: f32 = 0.010772;
const ASTEROID_PICKUP_DISTANCE_SQRD: f32 = 32. * 32.;
const ASTEROID_REATTACHMENT_TIMER: f32 = 0.1;
const DEAD_PLAYER_DETACHMENT_TIME: f32 = 1.0;

#[derive(Component)]
pub struct Asteroid;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Reflect, Default)]
pub enum AsteroidState {
    #[default]
    Attached,
    Flying,
    Inactive,
}

#[derive(Resource, Default)]
pub struct AsteroidReattachmentTimer(Timer);

fn asteroid_becoming_detached(
    mut asteroid_state: ResMut<NextState<AsteroidState>>,
    mut reattachment_timer: ResMut<AsteroidReattachmentTimer>,
) {
    asteroid_state.set(AsteroidState::Flying);
    reattachment_timer.0 = Timer::from_seconds(ASTEROID_REATTACHMENT_TIMER, TimerMode::Once);
}

fn detach_asteroid_from_dead_player(
    mut asteroid_state: ResMut<NextState<AsteroidState>>,
    mut reattachment_timer: ResMut<AsteroidReattachmentTimer>,
) {
    asteroid_state.set(AsteroidState::Flying);
    reattachment_timer.0 = Timer::from_seconds(DEAD_PLAYER_DETACHMENT_TIME, TimerMode::Once);
}

fn check_asteroid_becoming_inactive(
    asteroid_query: Query<&Velocity, With<Asteroid>>,
    mut state: ResMut<NextState<AsteroidState>>,
) {
    let velocity = asteroid_query.single();
    if velocity.0.length_squared() < INACTIVATION_SPEED_SQRD {
        state.set(AsteroidState::Inactive);
    }
}

fn check_asteroid_becoming_attached(
    asteroid_query: Query<&Position, With<Asteroid>>,
    player_query: Query<&Position, With<player::Player>>,
    mut state: ResMut<NextState<AsteroidState>>,
) {
    let position = asteroid_query.single();
    let player_position = player_query.single();
    if (*player_position - *position).length_squared() < ASTEROID_PICKUP_DISTANCE_SQRD {
        state.set(AsteroidState::Attached);
    }
}

fn on_asteroid_attached(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &mut Drag), With<Asteroid>>,
) {
    let (asteroid, mut drag) = asteroid_query.single_mut();
    commands
        .entity(asteroid)
        .insert(crate::movement::asteroid_movement::AsteroidMovement {
            gravity_multiplier: ASTEROID_GRAVITY_MULTIPLIER,
            repulsion_multiplier: ASTEROID_REPULSION_MULTIPLIER,
        });
    drag.0 = ASTEROID_DRAG;
}

fn on_asteroid_detached(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &mut Acceleration, &mut Drag), With<Asteroid>>,
) {
    let (asteroid, mut acceleration, mut drag) = asteroid_query.single_mut();
    commands
        .entity(asteroid)
        .remove::<crate::movement::asteroid_movement::AsteroidMovement>();
    acceleration.0 = Vec2::ZERO;
    drag.0 = DETACHED_ASTEROID_DRAG;
}

fn update_reattachment_timer(
    mut reattachment_timer: ResMut<AsteroidReattachmentTimer>,
    time: Res<Time>,
) {
    reattachment_timer.0.tick(time.delta());
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct AsteroidSchedule;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AsteroidState::Attached);
        app.insert_resource(AsteroidReattachmentTimer(Timer::default()));

        let mut asteroid_schedule = Schedule::new(AsteroidSchedule);
        asteroid_schedule.add_systems((
            check_asteroid_becoming_attached
                .run_if(|state: Res<State<AsteroidState>>| *state != AsteroidState::Attached)
                .run_if(|reattachment_timer: Res<AsteroidReattachmentTimer>| {
                    reattachment_timer.0.finished()
                }),
            asteroid_becoming_detached
                .run_if(|state: Res<State<AsteroidState>>| *state == AsteroidState::Attached)
                .run_if(on_event::<JustReleasingEvent>()),
            check_asteroid_becoming_inactive
                .run_if(|state: Res<State<AsteroidState>>| *state == AsteroidState::Flying),
            update_reattachment_timer
                .run_if(|state: Res<State<AsteroidState>>| *state != AsteroidState::Attached),
        ));
        app.add_schedule(asteroid_schedule);
        app.world
            .resource_mut::<FixedMainScheduleOrder>()
            .insert_after(FixedUpdate, AsteroidSchedule);

        app.add_systems(OnEnter(AsteroidState::Attached), on_asteroid_attached);
        app.add_systems(OnEnter(AsteroidState::Flying), on_asteroid_detached);
        app.add_systems(
            OnExit(crate::player::PlayerState::Alive),
            detach_asteroid_from_dead_player,
        );

        app.add_systems(
            Startup,
            |mut commands: Commands, asset_server: Res<AssetServer>| {
                commands.spawn((
                    crate::asteroid::Asteroid,
                    crate::game::components::GameComponentsBundle::new(
                        Vec2::new(0.00001, 0.),
                        crate::asteroid::ASTEROID_DRAG,
                    ),
                    crate::game::collider::CircleCollider { radius: 12.0 },
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..Default::default()
                        },
                        texture: asset_server.load("asteroid.png"),
                        transform: Transform::from_xyz(0., 0., 1.),

                        ..Default::default()
                    },
                ));
            },
        );
    }
}
