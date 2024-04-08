use std::default;

use crate::{explosion::ExplosionEvent, game::{collider::CircleCollider, components::Position}};
use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use crate::explosion::*;

pub const PLAYER_ACCELERATION: f32 = 1.375;
pub const PLAYER_DRAG: f32 = 0.0457297;
pub const FLASH_PERIOD: f32 = 0.2;

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

fn check_destructive_collision(
    player_query: Query<(&CircleCollider, &Position), With<Player>>,
    query: Query<(&CircleCollider, &Position), Or<(With<crate::enemy::Enemy>, With<Explosion>)>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    let (player_collider, player_position) = player_query.single();
    for (enemy_collider, enemy_position) in query.iter() {
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

fn set_player_state_timer(
    next_state: Res<State<PlayerState>>,
    mut timer: ResMut<PlayerStateTimer>,
) {
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

fn on_player_dead(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut crate::game::components::Acceleration, &mut Visibility), With<Player>>,
    mut explosion_event_writer: EventWriter<ExplosionEvent>,
) {
    let (player, mut acceleration, mut visibility) = player_query.single_mut();
    commands
        .entity(player)
        .remove::<crate::movement::input_movement::InputMovement>();
    acceleration.0 = Vec2::ZERO;
    visibility.set(Box::new(Visibility::Hidden)).unwrap();
    explosion_event_writer.send(ExplosionEvent(player));
}

fn on_player_invincible(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player = player_query.single();
    commands
        .entity(player)
        .insert(crate::movement::input_movement::InputMovement {
            speed: PLAYER_ACCELERATION,
        });
}

fn on_player_alive(mut player_query: Query<&mut Visibility, With<Player>>) {
    let mut visibility = player_query.single_mut();
    visibility.set(Box::new(Visibility::Visible)).unwrap();
}

fn flash_player_sprite(mut player_query: Query<&mut Visibility, With<Player>>, time: Res<Time>) {
    let mut visibility = player_query.single_mut();
    if time.elapsed_seconds() % FLASH_PERIOD < FLASH_PERIOD / 2. {
        visibility.set(Box::new(Visibility::Hidden)).unwrap();
    } else {
        visibility.set(Box::new(Visibility::Visible)).unwrap();
    }
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
            check_destructive_collision
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
        app.add_systems(OnEnter(PlayerState::Alive), on_player_alive);
        app.add_systems(Update, flash_player_sprite.run_if(|state: Res<State<PlayerState>>| *state == PlayerState::Invincible));

        app.add_systems(
            Startup,
            |mut commands: Commands,
             asset_server: Res<AssetServer>,
             mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                commands.spawn((
                    crate::player::Player,
                    crate::movement::input_movement::InputMovement {
                        speed: crate::player::PLAYER_ACCELERATION,
                    },
                    crate::sprite_updater::directional_updater::DirectionalUpdater { offset: 0 },
                    crate::game::components::GameComponentsBundle::new(
                        Vec2::ZERO,
                        crate::player::PLAYER_DRAG,
                    ),
                    CircleCollider { radius: 4.0 },
                    SpriteSheetBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..Default::default()
                        },
                        texture: asset_server.load("spaceship.png"),
                        atlas: TextureAtlas {
                            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                                Vec2::new(8., 8.),
                                13,
                                1,
                                None,
                                None,
                            )),
                            index: 0,
                        },
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..Default::default()
                    },
                ));
            },
        );
    }
}
