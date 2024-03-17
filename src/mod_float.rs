use core::ops::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ModFloat(pub f32);

const MODULO: f32 = 512.0;
const MODULO_HALF: f32 = MODULO / 2.0;

impl ModFloat {
    pub fn new(x: f32) -> Self {
        ModFloat((x + MODULO_HALF).rem_euclid(MODULO) - MODULO_HALF)
    }

    pub fn abs(&self) -> Self {
        if self.0 < 0.0 {
            ModFloat::new(-self.0)
        } else {
            *self
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Add for ModFloat {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ModFloat::new(self.0 + other.0)
    }
}

impl Sub for ModFloat {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ModFloat::new(self.0 - other.0)
    }
}

impl Mul for ModFloat {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ModFloat::new(self.0 * other.0)
    }
}

impl Div for ModFloat {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        ModFloat::new(self.0 / other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_addition() {
        assert_eq!(
            ModFloat::new(255.) + ModFloat::new(1.),
            ModFloat::new(-256.)
        );
    }

    #[test]
    fn test_mod_subtraction() {
        assert_eq!(
            ModFloat::new(-256.) - ModFloat::new(1.),
            ModFloat::new(255.)
        );
    }

    #[test]
    fn test_mod_multiplication() {
        assert_eq!(
            ModFloat::new(128.) * ModFloat::new(2.),
            ModFloat::new(-256.)
        );
    }

    #[test]
    fn test_mod_division() {
        assert_eq!(
            ModFloat::new(128.) / ModFloat::new(0.5),
            ModFloat::new(-256.)
        );
    }

    #[test]
    fn test_mod_abs() {
        assert_eq!(ModFloat::new(-128.).abs(), ModFloat::new(128.));
        assert_eq!(ModFloat::new(-256.).abs(), ModFloat::new(-256.));
    }

    #[test]
    fn test_new() {
        assert_eq!(ModFloat::new(256.), ModFloat::new(-256.));
    }

    #[test]
    fn test_value() {
        assert_eq!(ModFloat::new(256.).value(), -256.);
    }
}
