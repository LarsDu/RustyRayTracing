use super::vec3::Vec3;
use super::Point;

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray{
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        return Ray{origin, direction};
    }

    pub fn at(&self, t: f32) -> Point {
        return self.origin + self.direction * t;
    }
}