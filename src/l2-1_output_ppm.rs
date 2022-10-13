fn main(){
    print_example_ppm(256, 256);

}

fn print_example_ppm(image_width: i32, image_height: i32){
    println!("P3");
    println!("{image_width}");
    println!("{image_height}");
    println!("255\n");
    const SCALE_VAL: f32 = 255.99;
    for row in (0..image_height).rev(){
        eprintln!("Rows remaining: {}", image_height-row);
        for col in 0..image_width{
            let r: f32 = SCALE_VAL * (col as f32)/(image_width as f32 - 1.0);
            let g: f32 = SCALE_VAL * (row as f32)/(image_height as f32 - 1.0);
            let b: f32 = SCALE_VAL * 0.25;
            println!("{} {} {}\n",r as u8, g as u8, b as u8);
        }
    }
    eprintln!("\nDone.\n");
}