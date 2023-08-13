use image::Rgb;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const UNIT: Vec3 = Vec3(1.0, 1.0, 1.0);
    pub const RIGHT: Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const UP: Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const FORWARD: Vec3 = Vec3(0.0, 0.0, -1.0);

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3(self.1 * other.2 - self.2 * other.1,
             self.2 * other.0 - self.0 * other.2,
             self.0 * other.1 - self.1 * other.0)
    }

    pub fn len_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
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
        Rgb([value.0 as u8, value.1 as u8, value.2 as u8])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
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
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
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
        Vec3(other * self.0, other * self.1, other * self.2)
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
        Vec3(-self.0, -self.1, -self.2)
    }
}
