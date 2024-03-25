use bevy::prelude::*;

pub fn wrap(vec: Vec3) -> Vec3 {
    Vec3::new(
        (vec.x + 1024.).rem_euclid(2048.) - 1024.,
        (vec.y + 1024.).rem_euclid(2048.) - 1024.,
        (vec.z + 1024.).rem_euclid(2048.) - 1024.,
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