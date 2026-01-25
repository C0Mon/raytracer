#[doc(inline)]
use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use crate::math::util::{random_float, random_normalised};
#[derive(Debug, Clone, Copy, PartialEq)]
/// # 3D Vector
/// A digital representation of a 3D vector, with methods useful for raytracing
pub struct Vec3 {
    /// Stores the x coordinate of the vector
    pub x: f64,
    /// Stores the y coordinate of the vector
    pub y: f64,
    /// Stores the z coordinate of the vector
    pub z: f64,
}


impl Vec3 {
    /// Constructor used to create Vec3
    /// # Example
    /// ```
    /// use raytracer::math::vector::Vec3;
    /// 
    /// // This creates a vector called direction with x: 3.0, y: 4.0, and z: 5.0
    /// let direction = Vec3::new(3.0, 4.0, 5.0);
    /// ```
    pub fn new(x: f64, y:f64, z: f64) -> Self{
        Self {
            x,
            y,
            z
        }
    } 
    /// Calculates the dot product of itself and another vector
    /// # Example
    /// ```
    /// use raytracer::math::vector::Vec3;
    /// 
    /// let vec1 = Vec3::new(3.0, 4.0, 5.0);
    /// let vec2 = Vec3::new(7.0, 6.0, 5.0);
    /// 
    /// // Store dot product of vec1 and vec2
    /// let dot_product = vec1.dot(vec2);
    /// ```
    pub fn dot(&self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        return *self/self.length();
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1.0,1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt()
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(*normal) > 0.0 {
            return on_unit_sphere;
        };
        -on_unit_sphere
    }

    pub fn random() -> Self {
        Self { 
            x: random_normalised(),
            y: random_normalised(), 
            z: random_normalised(),
        }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self { 
            x: random_float(min, max),
            y: random_float(min, max), 
            z: random_float(min, max),
        }
    }
    
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self. x * other.y - self.y * other.x
        )
    }

    pub fn to_string(&self) -> String {
        format!("{0} ,   {1} ,   {2}", self.x, self.y, self.z)
    }

    
}

pub type Point3 = Vec3;

// Operator Overloads
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

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
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


impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> f64 {
        self.dot(rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
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
    #[case(Vec3::new(3.0, 4.0, 5.0), 10.0, Vec3::new(30.0, 40.0, 50.0))]
    #[case(Vec3::new(3.0, 2.0, 400.0), 0.0, Vec3::new(0.0, 0.0, 0.0))]
    #[case( Vec3::new(5.0, 4.0, 3.0), 4.0, Vec3::new(20.0, 16.0, 12.0))]
    fn test_scalar_mult(#[case] a: Vec3, #[case] b: f64, #[case] expected: Vec3) {
        assert_eq!(a*b, expected);
    }

    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), 10.0, Vec3::new(30.0, 40.0, 50.0))]
    #[case(Vec3::new(3.0, 2.0, 400.0), 0.0, Vec3::new(0.0, 0.0, 0.0))]
    #[case( Vec3::new(5.0, 4.0, 3.0), 4.0, Vec3::new(20.0, 16.0, 12.0))]
    fn test_scalar_mult_assign(#[case] mut a: Vec3, #[case] b: f64, #[case] expected: Vec3) {
        a *= b;
        assert_eq!(a, expected);
    }

    #[rstest]
    #[case(Vec3::new(30.0, 40.0, 50.0), 10.0, Vec3::new(3.0, 4.0, 5.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), 2.0, Vec3::new(0.0, 0.0, 0.0))]
    #[case(Vec3::new(20.0, 16.0, 12.0), 4.0, Vec3::new(5.0, 4.0, 3.0))]
    fn test_scalar_div(#[case] a: Vec3, #[case] b: f64, #[case] expected: Vec3) {
        assert_eq!(a/b, expected);
    }

    #[rstest]
    #[case(Vec3::new(30.0, 40.0, 50.0), 10.0, Vec3::new(3.0, 4.0, 5.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), 2.0, Vec3::new(0.0, 0.0, 0.0))]
    #[case(Vec3::new(20.0, 16.0, 12.0), 4.0, Vec3::new(5.0, 4.0, 3.0))]
    fn test_scalar_div_assign(#[case] mut a: Vec3, #[case] b: f64, #[case] expected: Vec3) {
        a /= b;
        assert_eq!(a, expected);
    }

    #[rstest]
    #[case(Vec3::new(30.0, 40.0, 50.0), Vec3::new(-30.0, -40.0, -50.0))]
    #[case(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))]
    #[case(Vec3::new(20.0, 16.0, 12.0), Vec3::new(-20.0, -16.0, -12.0))]
    fn test_negation(#[case] a: Vec3, #[case] expected: Vec3) {
        assert_eq!(-a, expected);
    }

    #[rstest]
    #[case(Vec3::new(3.0, 4.0, 5.0), Vec3::new(7.0, 6.0, 5.0), 70.0)]
    #[case(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), 32.0)]
    fn test_dot(#[case] a: Vec3, #[case] b: Vec3, #[case] expected: f64) {
        assert_eq!(a.dot(b), expected);
        assert_eq!(a*b, expected);
    }

    #[rstest]
    #[case(Vec3::new(5.0, 12.0, 13.0), Vec3::new(3.0, 4.0, 5.0), Vec3::new(8.0, 14.0, -16.0))]
    #[case(Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 4.0, 5.0), Vec3::new(-2.0, 4.0, -2.0))]
    #[case(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))]
    fn test_cross(#[case] a: Vec3, #[case] b: Vec3, #[case] expected: Vec3) {
        assert_eq!(a.cross(b), expected)
    }

    #[rstest]
    #[case(Vec3::new(5.0, 12.0, 13.0), Vec3::new(0.27196414,0.65271395,0.70710678))]
    #[case(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.26726124,0.53452248,0.80178372))]
    #[case(Vec3::new(-20.0, 40.0, 13.0), Vec3::new(-0.42943775,0.85887550,0.27913453))]
    fn test_unit_vector(#[case] a: Vec3, #[case] expected: Vec3) {
        let epsilon = 1e-6;
        let unit = a.unit_vector();
        assert!((unit.x - expected.x).abs() < epsilon);
        assert!((unit.y - expected.y).abs() < epsilon);
        assert!((unit.z - expected.z).abs() < epsilon);
    }

}

