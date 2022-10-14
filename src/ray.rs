use super::vec3::Vec3;
use super::Point;

pub struct Ray{
    origin: Vec3,
    direction: Vec3
}

impl Ray{
    fn new(&self, origin: Vec3, direction: Vec3) -> Self {
        return Ray{origin, direction};
    }

    fn at(&self, t: f32) -> Point {
        return self.origin + self.direction * t;
    }
}