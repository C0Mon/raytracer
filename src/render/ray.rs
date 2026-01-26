use crate::math::vector::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3 ) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn default() -> Self {
        Self {
            origin: Point3{ x: 0.0, y: 0.0, z: 0.0},
            direction: Vec3::default(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    } 
    pub fn direction(&self) -> Vec3 {
        self.direction 
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (t*self.direction)
    }
}
