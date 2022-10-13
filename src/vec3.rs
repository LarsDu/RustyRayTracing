#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) {
        Vec3 { x, y, z };
    }
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };



    // Method Squared
    pub fn length_squared(&self){

    }

    // Method Magnitude
    pub fn length(&self){

    }

    // Method Normalized
    pub fn normalize(self){

    }

    pub fn dot(a: &Vec3, b: &Vec3){
        
    }
    pub fn dot(self, rhs: Self) -> f32 {

    }

    pub fn cross(a: &Vec3, b: &Vec3){

    }

    pub fn cross(self, b: &Vec3) -> f32{

    }



    pub fn distance(self, rhs: Self) -> f32{
        (self - rhs).length_squared
    }



}



impl Mul<f32> for Vec3{}
impl Mul<Vec3> for Vec3{}

impl Add<Vec3> for Vec3{}

impl Sub<Vec3> for Vec3{}

impl Div<Vec3> for Vec3{

}

impl Div<Vec3> for f32{

}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32;3]) -> Self{
        Self::new(a[0], b[1], c[2]);
    }
}
