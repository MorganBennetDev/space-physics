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

use crate::vectors::{
    AbstractVector
};



pub struct Quaternion {
    vec: AbstractVector<4>,
    norm_value: Remember<f64>
}



impl Quaternion {
    pub fn mul(q1: AbstractVector<4>, q2: AbstractVector<4>) -> AbstractVector<4> {
        let [r1, i1, j1, k1] = q1.entries();
        let [r2, i2, j2, k2] = q2.entries();

        let r = r1 * r2 - i1 * i2 - j1 * j2 - k1 * k2;
        let i = r1 * i2 + i1 * r2 + j1 * k2 - k1 * j2;
        let j = r1 * j2 - i1 * k2 + j1 * r2 + k1 * i2;
        let k = r1 * k2 + i1 * j2 - j1 * i2 + k1 * r2;

        AbstractVector::new([r, i, j, k])
    }

    pub fn inv(q: AbstractVector<4>) -> AbstractVector<4> {
        let [r, i, j, k] = q.entries();

        let norm = r * r + i * i + j * j + k * k;

        AbstractVector::new([r / norm, -i / norm, -j / norm, -k / norm])
    }

    pub fn conj(u: AbstractVector<4>, v: AbstractVector<4>) -> AbstractVector<4> {
        Quaternion::mul(Quaternion::mul(u, v), Quaternion::inv(u))
    }



    pub fn new(r: f64, i: f64, j: f64, k: f64) -> Quaternion {
        let mut out = Quaternion { vec: AbstractVector::new([r, i, j, k]), norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || AbstractVector::<4>::norm_static(&out.vec));

        out.norm_value = Remember::new(calculate, Some(AbstractVector::<4>::norm_static(&out.vec)));

        out
    }

    pub fn new_with_norm(r: f64, i: f64, j: f64, k: f64, norm: f64) -> Quaternion {
        let mut out = Quaternion { vec: AbstractVector::new([r, i, j, k]), norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || AbstractVector::<4>::norm_static(&out.vec));

        out.norm_value = Remember::new(calculate, Some(norm));

        out
    }

    fn new_from_abstract_vector(vec: AbstractVector<4>) -> Quaternion {
        let [r, i, j, k] = vec.entries();

        Quaternion::new(*r, *i, *j, *k)
    }

    fn new_from_abstract_vector_with_norm(vec: AbstractVector<4>, norm: f64) -> Quaternion {
        let [r, i, j, k] = vec.entries();

        Quaternion::new_with_norm(*r, *i, *j, *k, norm)
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
        self.norm_value.get_static()
    }

    pub fn get(&self) -> &AbstractVector<4> {
        &self.vec
    }

    pub fn set(&mut self, q: AbstractVector<4>) {
        self.vec = q.clone();
        self.norm_value.stale = true;
    }

    pub fn inverse(&self) -> Quaternion {
        Quaternion::new_from_abstract_vector_with_norm(Quaternion::inv(self.vec), 1.0 / self.norm_value.get_static())
    }

    pub fn invert(&mut self) {
        self.vec = Quaternion::inv(self.vec);
        self.norm_value.set(1.0 / self.norm_value.get_static());
    }
}



impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion::new_from_abstract_vector(self.vec + other.vec)
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.vec += rhs.vec;
        self.norm_value.stale = true;
    }
}

impl Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, scalar: f64) -> Quaternion {
        Quaternion::new_from_abstract_vector_with_norm(self.vec / scalar, self.norm_value.get_static() / scalar)
    }
}

impl DivAssign<f64> for Quaternion {
    fn div_assign(&mut self, scalar: f64) {
        self.vec /= scalar;
        self.norm_value.set(self.norm_value.get_static() / scalar);
    }
}

impl Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, scalar: f64) -> Quaternion {
        Quaternion::new_from_abstract_vector_with_norm(self.vec * scalar, self.norm_value.get_static() * scalar)
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion::new_from_abstract_vector_with_norm(Quaternion::mul(self.vec, other.vec), self.norm_value.get_static() * other.norm_value.get_static())
    }
}

impl MulAssign<f64> for Quaternion {
    fn mul_assign(&mut self, scalar: f64) {
        self.vec *= scalar;
        self.norm_value.set(self.norm_value.get_static() * scalar);
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        self.norm_value.set(self.norm_value.get_static() * rhs.norm_value.get_static());
        self.vec = Quaternion::mul(self.vec, rhs.vec);
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Quaternion {
        Quaternion::new_from_abstract_vector_with_norm(-self.vec, self.norm_value.get_static())
    }
}

impl Not for Quaternion {
    type Output = Quaternion;

    fn not(self) -> Quaternion {
        Quaternion::new_with_norm(
            self.vec[0],
            -self.vec[1],
            -self.vec[2],
            -self.vec[3],
            self.norm_value.get_static()
        )
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion::new_from_abstract_vector(self.vec - other.vec)
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        self.vec -= rhs.vec;
        self.norm_value.stale = true;
    }
}