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

use super::Quaternion;



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

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        let x = u.y * v.z - u.z * v.y;
        let y = u.z * v.x - u.x * v.z;
        let z = u.x * v.y - u.y * v.x;

        Vec3::new(x, y, z)
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

impl Mul<Quaternion> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Quaternion) -> Vec3 {
        let (r, i, j, k) = Quaternion::conj(
            (0.0, self.x, self.y, self.z),
            other.get()
        );

        Vec3::new(i, j, k)
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

impl MulAssign<Quaternion> for Vec3 {
    fn mul_assign(&mut self, other: Quaternion) {
        let (r, i, j, k) = Quaternion::conj(
            (0.0, self.x, self.y, self.z),
            other.get()
        );

        self.x = i;
        self.y = j;
        self.z = k;
        self.norm_value.stale = true;
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