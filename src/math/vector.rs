use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};
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

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    
    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), Vec3::new(7.0, 6.0, 5.0), Vec3::new(10.0, 10.0, 10.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0))]
    fn test_add(#[case] a: Vec3, #[case] b: Vec3, #[case] expected: Vec3) {
        assert_eq!(a + b, expected);
    }

    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), Vec3::new(7.0, 6.0, 5.0), Vec3::new(10.0, 10.0, 10.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0))]
    fn test_add_assign(#[case] mut a: Vec3, #[case] b: Vec3, #[case] expected: Vec3) {
        a += b;
        assert_eq!(a, expected);
    }


    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), Vec3::new(7.0, 6.0, 5.0), Vec3::new(-4.0, -2.0, 0.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0))]
    fn test_sub(#[case] a: Vec3, #[case] b: Vec3, #[case] expected: Vec3) {
        assert_eq!(a - b, expected);
    }

    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), Vec3::new(7.0, 6.0, 5.0), Vec3::new(-4.0, -2.0, 0.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0))]
    fn test_sub_assign(#[case] mut a: Vec3, #[case] b: Vec3, #[case] expected: Vec3) {
        a -= b;
        assert_eq!(a, expected);
    }

    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), Vec3::new(7.0, 6.0, 5.0), 70.0)]
    #[case(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), 32.0)]
    fn test_dot(#[case] a: Vec3, #[case] b: Vec3, #[case] expected: f64) {
        assert_eq!(a.dot(b), expected);
        assert_eq!(a*b, expected);
    }
}

