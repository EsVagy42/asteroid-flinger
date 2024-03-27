use bevy::prelude::*;

const MODULO: f32 = 2048.;
const MODULO_HALF: f32 = MODULO / 2.;

pub fn wrap(vec: Vec3) -> Vec3 {
    Vec3::new(
        (vec.x + MODULO_HALF).rem_euclid(MODULO) - MODULO_HALF,
        (vec.y + MODULO_HALF).rem_euclid(MODULO) - MODULO_HALF,
        (vec.z + MODULO_HALF).rem_euclid(MODULO) - MODULO_HALF,
    )
}

#[cfg(test)]
mod wrap_tests {
    use super::*;

    #[test]
    fn test_vec2() {
        assert_eq!(wrap(Vec3::new(-1024., -1024., -1024.) - Vec3::new(1., 1., 1.)), Vec3::new(1023., 1023., 1023.));
    }
}