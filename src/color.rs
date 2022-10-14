pub use super::vec3::Vec3 as Color;

const COLOR_SCALE_VAL: f32 = 255.99;

pub fn write_color(color: Color){
    let r: f32 = COLOR_SCALE_VAL * color.x;
    let g: f32 = COLOR_SCALE_VAL * color.y;
    let b: f32 = COLOR_SCALE_VAL * color.z;
    println!("{} {} {}\n",r as u8, g as u8, b as u8);
}