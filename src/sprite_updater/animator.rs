use bevy::prelude::*;

#[derive(Component)]
pub struct Animator {
    first_index: usize,
    last_index: usize,
    current_index: usize,
    looping: bool,
    timer: Timer,
}

impl Animator {
    pub fn new(first_index: usize, last_index: usize, delay: f32, looping: bool) -> Self {
        Animator {
            first_index,
            last_index,
            current_index: first_index,
            looping,
            timer: Timer::from_seconds(delay, TimerMode::Repeating),
        }
    }
}

pub fn update (
    mut query: Query<(&mut TextureAtlas, &mut Animator)>,
    time: Res<Time>,
) {
    for (mut texture_atlas, mut animator) in query.iter_mut() {
        animator.timer.tick(time.delta());
        if animator.timer.just_finished() {
            animator.current_index += 1;
            if animator.current_index > animator.last_index {
                if animator.looping {
                    animator.current_index = animator.first_index;
                } else {
                    animator.current_index = animator.last_index;
                }
            }
        }
        texture_atlas.index = animator.current_index;
    }
}

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}