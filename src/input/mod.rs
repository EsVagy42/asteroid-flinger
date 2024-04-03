use bevy::{app::FixedMainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

#[derive(Resource)]
pub struct GameInput {
    pub direction: Vec2,
}

impl Default for GameInput {
    fn default() -> Self {
        Self {
            direction: Vec2::ZERO,
        }
    }
}

#[derive(Event)]
pub struct JustReleasingEvent;

fn update_input(
    mut input: ResMut<GameInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut releasing_event_writer: EventWriter<JustReleasingEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        releasing_event_writer.send(JustReleasingEvent);
    }
    
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    input.direction = direction.normalize_or_zero();
}

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct InputSchedule;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JustReleasingEvent>();
        app.insert_resource(GameInput::default());
        let mut input_schedule = Schedule::new(InputSchedule);
        input_schedule.add_systems(update_input);
        app.add_schedule(input_schedule);
        app.world.resource_mut::<FixedMainScheduleOrder>().insert_after(FixedUpdate, InputSchedule);
    }
}