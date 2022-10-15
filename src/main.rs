mod vec3;
use vec3::Vec3;
use vec3::Vec3 as Point;

mod color;
use color::{write_color, Color};

mod ray;
use ray::Ray;

mod camera;
use camera::Camera;
fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32/ASPECT_RATIO) as u32;
    eprintln!("Image width: {IMAGE_WIDTH}\tImage height{IMAGE_HEIGHT}");
    // Camera
    let camera = Camera::default();
    // Render
    render(IMAGE_WIDTH, IMAGE_HEIGHT, &camera);
}

fn hit_sphere(center: Point, radius: f32, r: &Ray) -> bool{
    let oc: Vec3 = r.origin - center;
    let a: f32 = Vec3::dot_product(&r.direction, &r.direction);
    let b = Vec3::dot_product(&oc, &r.direction) * 2.0;
    let c: f32 = Vec3::dot_product(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    // Calculate unit direction
    let unit_direction: Vec3 = Vec3::normalized(r.direction);
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn render(image_width: u32, image_height: u32, camera: &Camera) {
    println!("P3");
    println!("{image_width}");
    println!("{image_height}");
    println!("255\n");
    for row in (0..image_height).rev() {
        eprintln!("Rows remaining: {}", row);
        for col in 0..image_width {
            let u: f32 = (col as f32) / (image_width as f32 - 1.0);
            let v: f32 = (row as f32) / (image_height as f32 - 1.0);       
            // Cast a ray for every pixel in the camera
            let r = Ray::new(
                camera.origin,
                camera.lower_left_corner + camera.horizontal * u + camera.vertical * v - camera.origin
            );
            write_color(
                ray_color(&r)
            );
        }
    }
    eprintln!("\nDone.\n");
}
