mod vec3;
use vec3::Vec3;
use vec3::Vec3 as Point;

mod color;
use color::{write_color, Color};

mod ray;
use ray::Ray;

fn main() {
    print_example_ppm(256, 256);
}

fn print_example_ppm(image_width: i32, image_height: i32) {
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
