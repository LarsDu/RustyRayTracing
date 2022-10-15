use super::Point;
use super::Vec3;
use super::Ray;

pub struct HitRecord{
    pub p: Point,
    pub normal: Vec3,
    pub t: f32    
}

impl Default for HitRecord{
    fn default() -> Self {
        return HitRecord{
            p: Point::ZERO,
            normal: Vec3::ONE,
            t: 1.0
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

