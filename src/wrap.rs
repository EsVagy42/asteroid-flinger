use bevy::prelude::*;

pub fn wrap(vec: Vec3) -> Vec3 {
    Vec3::new(
        (vec.x + 512.).rem_euclid(1024.) - 512.,
        (vec.y + 512.).rem_euclid(1024.) - 512.,
        (vec.z + 512.).rem_euclid(1024.) - 512.,
    )
}

#[cfg(test)]
mod wrap_tests {
    use super::*;

    #[test]
    fn test_vec2() {
        assert_eq!(wrap(Vec3::new(-512., -512., -512.) - Vec3::new(1., 1., 1.)), Vec3::new(511., 511., 511.));
    }
}