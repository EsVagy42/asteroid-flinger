use bevy::prelude::*;

pub const MODULO: f32 = 2048.;
pub const MODULO_HALF: f32 = MODULO / 2.;

pub fn wrap_f32(x: f32) -> f32 {
    (x + MODULO_HALF).rem_euclid(MODULO) - MODULO_HALF
}

pub fn wrap_vec2(v: Vec2) -> Vec2 {
    Vec2::new(wrap_f32(v.x), wrap_f32(v.y))
}

#[cfg(test)]
mod wrap_tests {
    use super::*;

    #[test]
    fn test_vec2() {
        assert_eq!(wrap_vec2(Vec2::new(-1024., -1024.) - Vec2::new(1., 1.)), Vec2::new(1023., 1023.));
    }
}