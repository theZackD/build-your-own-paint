use std::fmt;
use std::ops::{Add, Mul, Sub};

/// A 2D vector, used for positions and sizes on screen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns a vector of length 1 pointing in the same direction.
    /// Fails for the zero vector, which has no direction.
    #[allow(dead_code)] // Chapter 1 exercise, not used by the app itself.
    pub fn normalize(self) -> Result<Vec2, String> {
        let len = self.length();
        if len == 0.0 {
            Err(String::from("cannot normalize the zero vector"))
        } else {
            Ok(Vec2::new(self.x / len, self.y / len))
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f32) -> Vec2 {
        Vec2::new(self.x * k, self.y * k)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Vec2 {
        Vec2::new(x, y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Returns the longest vector of the slice, or `None` if it is empty.
#[allow(dead_code)] // Chapter 1 exercise, not used by the app itself.
pub fn longest(points: &[Vec2]) -> Option<Vec2> {
    let mut best: Option<Vec2> = None;
    for &p in points {
        match best {
            Some(b) if b.length() >= p.length() => {}
            _ => best = Some(p),
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 5.0);
        assert_eq!(a + b, Vec2::new(4.0, 7.0));
        assert_eq!(b - a, Vec2::new(2.0, 3.0));
        assert_eq!(a * 2.0, Vec2::new(2.0, 4.0));
        assert_eq!(a.dot(b), 13.0);
    }

    #[test]
    fn normalize() {
        assert_eq!(Vec2::new(3.0, 4.0).normalize(), Ok(Vec2::new(0.6, 0.8)));
        assert!(Vec2::ZERO.normalize().is_err());
    }

    #[test]
    fn longest_vector() {
        assert_eq!(longest(&[]), None);
        let pts = [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 3.0),
            Vec2::new(2.0, 0.0),
        ];
        assert_eq!(longest(&pts), Some(Vec2::new(0.0, 3.0)));
    }
}
