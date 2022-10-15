use core::ops::*;
#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
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

    // Vector dot product
    pub fn dot_product(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        Self::dot_product(self, rhs)
    }
    // Method Squared
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    // Method Magnitude
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    // Method Normalized
    pub fn normalized(self) -> Self {
        return self / self.length();
    }

    pub fn cross_product(a: &Vec3, b: &Vec3) -> Self {
        Self {
            x: a.y * b.z - b.y * a.z,
            y: a.z * b.x - b.z * a.x,
            z: a.x * b.y - b.x * a.y,
        }
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self::cross_product(&self, rhs)
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        return Self::new(a[0], a[1], a[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        assert_eq!(
            52.0,
            Vec3::dot_product(&Vec3::new(3.0, 4.0, 5.0), &Vec3::new(2.0, 4.0, 6.0))
        );
    }
}
