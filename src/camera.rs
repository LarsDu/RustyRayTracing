use std::default::Default;
use super::Point;
use super::Vec3;


pub struct Camera{
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    pub origin: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3
}

impl Default for Camera{
    fn default() -> Self{
        let origin = Point::ZERO;
        let aspect_ratio: f32 = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let focal_length= 1.0;
        Camera { 
            origin: origin, 
            aspect_ratio: aspect_ratio, 
            viewport_height: viewport_height, 
            viewport_width: viewport_width, 
            focal_length: focal_length, 
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal * 0.5 - vertical * 0.5 - Vec3::new(0.0, 0.0, focal_length)
        }
    }

}