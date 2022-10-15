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
    const aspect_ratio: f32 = 16.0/9.0;
    const image_width: u32 = 400;
    const image_height: u32 = image_width/aspect_ratio as u32;
    // Camera
    let camera = Camera::default();
    // Render
    render(image_width, image_height, &camera);
}

fn ray_color(ray: &Ray) -> Color {
    // Calculate unit direction
    let unit_direction: Vec3 = Vec3::normalized(ray.direction);
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn render(image_width: u32, image_height: u32, camera: &Camera) {
    println!("P3");
    println!("{image_width}");
    println!("{image_height}");
    println!("255\n");
    for row in (0..image_height).rev() {
        eprintln!("Rows remaining: {}", image_height - row);
        for col in 0..image_width {
            let u: f32 = (row as f32) / (image_width as f32 - 1.0);
            let v: f32 = (col as f32) / (image_height as f32 - 1.0);
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
