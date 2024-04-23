use super::*;
use bevy::prelude::*;

#[derive(Bundle)]
struct Follower {
    enemy: Enemy,
}
