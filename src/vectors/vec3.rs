use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign
};

use crate::util::Remember;



pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
    norm_value: Remember<f64>
}



impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        let mut out = Vec3 { x, y, z, norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || {
            (out.x * out.x + out.y * out.y + out.z * out.z).sqrt()
        });

        out.norm_value = Remember::new(calculate, Some(out.norm()));

        out
    }

    fn new_with_norm(x: f64, y: f64, z: f64, norm: f64) -> Vec3 {
        let mut out = Vec3 { x, y, z, norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || {
            (out.x * out.x + out.y * out.y + out.z * out.z).sqrt()
        });

        out.norm_value = Remember::new(calculate, Some(norm));

        out
    }

    pub fn norm(&mut self) -> f64 {
        self.norm_value.get()
    }
}



impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.norm_value.stale = true;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new_with_norm(self.x / scalar, self.y / scalar, self.z / scalar, self.norm_value.get_static() / scalar)
    }

}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;

        let norm = self.norm();
        self.norm_value.set(norm / scalar);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3::new_with_norm(self.x * scalar, self.y * scalar, self.z * scalar, self.norm_value.get_static() * scalar)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;

        let norm = self.norm();
        self.norm_value.set(norm * scalar);
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new_with_norm(-self.x, -self.y, -self.z, self.norm_value.get_static())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.norm_value.stale = true;
    }
}