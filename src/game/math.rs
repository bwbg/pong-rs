//! Definition of the math necessary to represent a Pong game. This will be
//! mainly geometry and linear algebra.
use std::ops::{Add, Mul};

/// Generic two dimensional vector within a Cartesian coordinate system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2f(f32, f32);

impl Add for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Testing
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() {
        let a = Vec2f(12.0, 34.0);
        let b = Vec2f(56.0, 78.0);
        assert_eq!(a + b, Vec2f(68.0, 112.0));
    }

    #[test]
    fn vector_scaling() {
        let a = Vec2f(12.0, 34.0);
        assert_eq!(a * 3.0, Vec2f(36.0, 102.0));
    }
}
