use bevy::prelude::*;

#[derive(Resource)]
pub struct GameInput {
    pub direction: Vec2,
    pub sprite_modifier: InputSpriteModifier,
}

impl Default for GameInput {
    fn default() -> Self {
        Self {
            direction: Vec2::ZERO,
            sprite_modifier: InputSpriteModifier::default(),
        }
    }
}

#[derive(Resource)]
pub struct InputSpriteModifier {
    pub flip_x: bool,
    pub flip_y: bool,
    pub index: usize,
}

impl Default for InputSpriteModifier {
    fn default() -> Self {
        Self {
            flip_x: false,
            flip_y: false,
            index: 0,
        }
    }
}

impl InputSpriteModifier {
    pub fn apply_to_sprite(&self, sprite: &mut Sprite) {
        sprite.flip_x = self.flip_x;
        sprite.flip_y = self.flip_y;
    }

    pub fn apply_to_atlas(&self, atlas: &mut TextureAtlas) {
        atlas.index = self.index;
    }
}

pub fn update_input(
    mut input: ResMut<GameInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW)
    {
        direction += Vec2::new(1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS)
    {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA)
    {
        direction += Vec2::new(0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD)
    {
        direction += Vec2::new(0.0, 1.0);
    }
    
    input.direction = direction.normalize_or_zero();
    if direction == Vec2::ZERO {
        return;
    }

    input.sprite_modifier.index = match direction {
        direction if direction.length() > 1. => 1,
        direction if direction.x == 0. => 2,
        _ => 0,
    };

    input.sprite_modifier.flip_x = direction.y >= 0.;
    input.sprite_modifier.flip_y = direction.x < 0.;
}