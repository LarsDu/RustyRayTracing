
use super::Point;
use super::hittable::*;
use super::vec3::*;
use super::ray::*;


pub struct Sphere{
    pub center: Point,
    pub radius: f32,
}

impl Sphere{
    fn new(center: Point, radius:f32) -> Self {
        return Sphere{
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot_product(&oc, &r.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b * half_b - a*c;
        if discriminant < 0.0{
            return false
        }
        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest square root that lies in the acceptable range
        let mut root = (-half_b - sqrtd)/a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd)/a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center)/self.radius;
        return true;
        
    }
}