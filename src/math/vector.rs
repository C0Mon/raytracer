use std::ops::{Add, Sub};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y:f64, z: f64) -> Self{
        Self {
            x,
            y,
            z
        }
    } 
    pub fn dot(&self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(3f64, 4f64, 5f64);
        let vec2 = Vec3::new(7f64, 6f64, 5f64);

        let expected = Vec3::new(10f64, 10f64, 10f64);
        let result = vec1 + vec2;

        assert_eq!(expected, result)
    }
    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(3f64, 4f64, 5f64);
        let vec2 = Vec3::new(7f64, 6f64, 5f64);

        let expected = Vec3::new(-4f64, -2f64, 0f64);
        let result = vec1 - vec2;

        assert_eq!(expected, result)
    }
    #[test]
    fn test_dot() {
        let vec1 = Vec3::new(3f64, 4f64, 5f64);
        let vec2 = Vec3::new(7f64, 6f64, 5f64);

        let expected = 70f64;
        let result = vec1.dot(vec2);

        assert_eq!(expected, result)
    }
}
