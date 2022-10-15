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

    // Render
    render(image_width, image_height);
}

fn render(image_width: u32, image_height: u32) {
    println!("P3");
    println!("{image_width}");
    println!("{image_height}");
    println!("255\n");
    for row in (0..image_height).rev() {
        eprintln!("Rows remaining: {}", image_height - row);
        for col in 0..image_width {
            write_color(
                Color::new(
                    (row as f32) / (image_width as f32 - 1.0),
                    (col as f32) / (image_height as f32 - 1.0),
                    0.25,
                )
            );
        }
    }
    eprintln!("\nDone.\n");
}
