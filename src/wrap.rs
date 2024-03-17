use bevy::prelude::*;

fn wrap(vec: Vec3) -> Vec3 {
    Vec3::new(
        (vec.x + 256.).rem_euclid(512.) - 256.,
        (vec.y + 256.).rem_euclid(512.) - 256.,
        (vec.z + 256.).rem_euclid(512.) - 256.,
    )
}

#[cfg(test)]
mod wrap_tests {
    use super::*;

    #[test]
    fn test_vec2() {
        assert_eq!(wrap(Vec3::new(-256., -256., -256.) - Vec3::new(1., 1., 1.)), Vec3::new(255., 255., 255.));
    }
}