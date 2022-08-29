use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Not,
    Sub,
    SubAssign
};

use crate::util::Remember;

pub struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
    norm_value: Remember<f64>
}



impl Quaternion {
    pub fn new(r: f64, i: f64, j: f64, k: f64) -> Quaternion {
        let mut out = Quaternion { r, i, j, k, norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || {
            (out.r * out.r + out.i * out.i + out.j * out.j + out.k * out.k).sqrt()
        });

        out.norm_value = Remember::new(calculate, Some(out.norm()));

        out
    }

    pub fn new_with_norm(r: f64, i: f64, j: f64, k: f64, norm: f64) -> Quaternion {
        let mut out = Quaternion { r, i, j, k, norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || {
            (out.r * out.r + out.i * out.i + out.j * out.j + out.k * out.k).sqrt()
        });

        out.norm_value = Remember::new(calculate, Some(norm));

        out
    }

    pub fn norm(&mut self) -> f64 {
        self.norm_value.get()
    }

    pub fn get(&self) -> (f64, f64, f64, f64) {
        (self.r, self.i, self.j, self.k)
    }

    pub fn set(&mut self, r: f64, i: f64, j: f64, k: f64) {
        self.r = r;
        self.i = i;
        self.j = j;
        self.k = k;
        self.norm_value.stale = true;
    }
}



impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion::new(
            self.r + other.r,
            self.i + other.i,
            self.j + other.j,
            self.k + other.k
        )
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
        self.j += rhs.j;
        self.k += rhs.k;
        self.norm_value.stale = true;
    }
}

impl Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, scalar: f64) -> Quaternion {
        Quaternion::new_with_norm(
            self.r / scalar,
            self.i / scalar,
            self.j / scalar,
            self.k / scalar,
            self.norm_value.get_static() / scalar
        )
    }
}

impl DivAssign<f64> for Quaternion {
    fn div_assign(&mut self, scalar: f64) {
        self.r /= scalar;
        self.i /= scalar;
        self.j /= scalar;
        self.k /= scalar;

        let norm = self.norm_value.get();
        self.norm_value.set(norm / scalar);
    }
}

impl Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, scalar: f64) -> Quaternion {
        Quaternion::new_with_norm(
            self.r * scalar,
            self.i * scalar,
            self.j * scalar,
            self.k * scalar,
            self.norm_value.get_static() * scalar
        )
    }
}

impl MulAssign<f64> for Quaternion {
    fn mul_assign(&mut self, scalar: f64) {
        self.r *= scalar;
        self.i *= scalar;
        self.j *= scalar;
        self.k *= scalar;

        let norm = self.norm_value.get();
        self.norm_value.set(norm * scalar);
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Quaternion {
        Quaternion::new_with_norm(
            -self.r,
            -self.i,
            -self.j,
            -self.k,
            self.norm_value.get_static()
        )
    }
}

impl Not for Quaternion {
    type Output = Quaternion;

    fn not(self) -> Quaternion {
        Quaternion::new_with_norm(
            self.r,
            -self.i,
            -self.j,
            -self.k,
            self.norm_value.get_static()
        )
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion::new(
            self.r - other.r,
            self.i - other.i,
            self.j - other.j,
            self.k - other.k
        )
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
        self.j -= rhs.j;
        self.k -= rhs.k;
        self.norm_value.stale = true;
    }
}