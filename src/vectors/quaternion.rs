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



type simple_quaternion = (f64, f64, f64, f64);



pub struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
    norm_value: Remember<f64>
}



impl Quaternion {
    pub fn mul(q1: simple_quaternion, q2: simple_quaternion) -> simple_quaternion {
        let (r1, i1, j1, k1) = q1;
        let (r2, i2, j2, k2) = q2;

        let r = r1 * r2 - i1 * i2 - j1 * j2 - k1 * k2;
        let i = r1 * i2 + i1 * r2 + j1 * k2 - k1 * j2;
        let j = r1 * j2 - i1 * k2 + j1 * r2 + k1 * i2;
        let k = r1 * k2 + i1 * j2 - j1 * i2 + k1 * r2;

        (r, i, j, k)
    }

    pub fn inv(q: simple_quaternion) -> simple_quaternion {
        let (r, i, j, k) = q;

        let norm = r * r + i * i + j * j + k * k;

        (r / norm, -i / norm, -j / norm, -k / norm)
    }

    pub fn conj(u: simple_quaternion, v: simple_quaternion) -> simple_quaternion {
        let u_inv = Quaternion::inv(u);

        Quaternion::mul(Quaternion::mul(u, v), u_inv)
    }



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

    pub fn new_from_axis_angle(axis: (f64, f64, f64), angle: f64) -> Quaternion {
        let (x, y, z) = axis;

        let norm = (x * x + y * y + z * z).sqrt();

        let x = x / norm;
        let y = y / norm;
        let z = z / norm;

        let r = (angle / 2.0).cos();
        let i = (angle / 2.0).sin() * x;
        let j = (angle / 2.0).sin() * y;
        let k = (angle / 2.0).sin() * z;

        Quaternion::new(r, i, j, k)
    }



    pub fn norm(&mut self) -> f64 {
        self.norm_value.get()
    }

    pub fn get(&self) -> simple_quaternion {
        (self.r, self.i, self.j, self.k)
    }

    pub fn set(&mut self, q: simple_quaternion) {
        let (r, i, j, k) = q;

        self.r = r;
        self.i = i;
        self.j = j;
        self.k = k;
        self.norm_value.stale = true;
    }

    pub fn inverse(&self) -> Quaternion {
        let (r, i, j, k) = Quaternion::inv(self.get());
        let norm = 1.0 / self.norm_value.get_static();

        Quaternion::new_with_norm(r, i, j, k, norm)
    }

    pub fn invert(&mut self) {
        let (r, i, j, k) = Quaternion::inv(self.get());
        let norm = 1.0 / self.norm();

        self.r = r;
        self.i = i;
        self.j = j;
        self.k = k;
        self.norm_value.set(norm);
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

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        let (r, i, j, k) = Quaternion::mul(self.get(), other.get());

        Quaternion::new_with_norm(
            r, i, j, k,
            self.norm_value.get_static() * other.norm_value.get_static()
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

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        let r = self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k;
        let i = self.r * rhs.i + rhs.r * self.i + self.j * rhs.k - self.k * rhs.j;
        let j = self.r * rhs.j + rhs.r * self.j + self.k * rhs.i - self.i * rhs.k;
        let k = self.r * rhs.k + rhs.r * self.k + self.i * rhs.j - self.j * rhs.i;

        self.r = r;
        self.i = i;
        self.j = j;
        self.k = k;

        let norm = self.norm_value.get();
        self.norm_value.set(norm * rhs.norm_value.get_static());
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