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

use std::cmp::{
    Eq,
    PartialEq
};

use crate::util::Remember;

use crate::vectors::{
    AbstractVector,
    Quaternion
};



pub struct Vector<const N: usize> {
    vec: AbstractVector<N>,
    norm_value: Remember<f64>
}



impl<const N: usize> Vector<N> {
    pub fn new(coordinates: [f64; N]) -> Vector<N> {
        Vector::new_from_abstract_vector(AbstractVector::new(coordinates))
    }

    fn new_with_norm(coordinates: [f64; N], norm: f64) -> Vector<N> {
        Vector::new_from_abstract_vector_with_norm(AbstractVector::new(coordinates), norm)
    }

    fn new_from_abstract_vector(vec: AbstractVector<N>) -> Vector<N> {
        let mut out = Vector { vec: vec.clone(), norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || AbstractVector::<N>::norm_static(&out.vec));

        out.norm_value = Remember::new(calculate, Some(AbstractVector::<N>::norm_static(&out.vec)));

        out
    }

    fn new_from_abstract_vector_with_norm(vec: AbstractVector<N>, norm: f64) -> Vector<N> {
        let mut out = Vector { vec: vec.clone(), norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(move || AbstractVector::<N>::norm_static(&out.vec));

        out.norm_value = Remember::new(calculate, Some(norm));

        out
    }

    pub fn norm(&self) -> f64 {
        self.vec.norm()
    }
}



impl Vector<3> {
    pub fn cross(u: Vector<3>, v: Vector<3>) -> Vector<3> {
        let [ux, uy, uz] = u.vec.entries();
        let [vx, vy, vz] = v.vec.entries();

        let x = ux * vz - uz * vy;
        let y = uz * vx - ux * vz;
        let z = ux * vy - uy * vx;

        Vector::new([x, y, z])
    }
}



impl<const N: usize> Add for Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Vector<N> {
        Vector::new((self.vec + other.vec).entries().clone())
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.vec += rhs.vec;

        self.norm_value.stale = true;
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, scalar: f64) -> Vector<N> {
        Vector::new_with_norm(self.vec.entries().clone(), self.norm() / scalar)
    }

}

impl<const N: usize> DivAssign<f64> for Vector<N> {
    fn div_assign(&mut self, scalar: f64) {
        self.vec /= scalar;

        let norm = self.norm();
        self.norm_value.set(norm / scalar);
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector::new_with_norm(self.vec.entries().clone(), self.norm() * scalar)
    }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = f64;

    fn mul(self, other: Vector<N>) -> f64 {
        self.vec * other.vec
    }
}

impl Mul<Quaternion> for Vector<3> {
    type Output = Vector<3>;

    fn mul(self, other: Quaternion) -> Vector<3> {
        let c = Quaternion::conj(
            AbstractVector::new([0.0, self.vec[0], self.vec[1], self.vec[2]]),
            *other.get()
        );

        Vector::new([c[1], c[2], c[3]])
    }
}

impl<const N: usize> MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, scalar: f64) {
        self.vec *= scalar;

        let norm = self.norm();
        self.norm_value.set(norm * scalar);
    }
}

impl MulAssign<Quaternion> for Vector<3> {
    fn mul_assign(&mut self, other: Quaternion) {
        let c = Quaternion::conj(
            AbstractVector::new([0.0, self.vec[0], self.vec[1], self.vec[2]]),
            *other.get()
        );

        self.vec = AbstractVector::new([c[1], c[2], c[3]]);
        self.norm_value.stale = true;
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Vector<N> {
        Vector::new_with_norm(self.vec.entries().clone(), self.norm())
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: Vector<N>) -> Vector<N> {
        Vector::new((self.vec - other.vec).entries().clone())
    }

}

impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, other: Vector<N>) {
        self.vec -= other.vec;

        self.norm_value.stale = true;
    }
}



impl<const N: usize> Eq for Vector<N> {}

impl<const N: usize> PartialEq for Vector<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.vec[i] != other.vec[i] {
                return false;
            }
        }

        true
    }
}