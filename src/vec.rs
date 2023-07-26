use image::Rgb;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const UNIT: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const UP: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const RIGHT: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const FORWARD: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3::new(self.y * other.z - self.z * other.y,
             self.z * other.x - self.x * other.z,
             self.x * other.y - self.y * other.x)
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        *self * (1.0 / self.len())
    }
}

impl From<Vec3> for Rgb<u8> {
    fn from(mut value: Vec3) -> Self {
        value *= 255.0;
        Rgb([value.x as u8, value.y as u8, value.z as u8])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(other * self.x, other * self.y, other * self.z)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
