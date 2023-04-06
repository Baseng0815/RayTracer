use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(self.1 * other.2 - self.2 * other.1,
             self.2 * other.0 - self.0 * other.2,
             self.0 * other.1 - self.1 * other.0)
    }

    pub fn len_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.len_squared().sqrt();
        *self * (1.0 / len)
    }
}

impl From<Vec3> for Rgb<u8> {
    fn from(mut value: Vec3) -> Rgb<u8> {
        value *= 255.0;
        Rgb([value.0 as u8, value.1 as u8, value.2 as u8])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        other * self
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = other * *self
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}
