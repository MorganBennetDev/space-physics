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



pub struct Vector<const N: usize> {
    coordinates: [f64; N],
    norm_value: Remember<f64>
}



impl<const N: usize> Vector<N> {
    pub fn new(coordinates: [f64; N]) -> Vector<N> {
        let mut out = Vector { coordinates, norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(|| out.norm());

        out.norm_value = Remember::new(calculate, Some(Vector::<N>::norm_static(coordinates)));

        out
    }

    fn new_with_norm(coordinates: [f64; N], norm: f64) -> Vector<N> {
        let mut out = Vector { coordinates, norm_value: Remember::new(Box::new(|| { 0.0 }), Some(0.0)) };

        let calculate = Box::new(|| out.norm());

        out.norm_value = Remember::new(calculate, Some(Vector::<N>::norm_static(coordinates)));

        out
    }

    pub fn norm(&self) -> f64 {
        Vector::<N>::norm_static(self.coordinates)
    }

    fn norm_static(coordinates: [f64; N]) -> f64 {
        let norm = 0.0;

        for c in coordinates {
            norm += c * c;
        }

        norm.sqrt()
    }
}



impl Vector<3> {
    pub fn cross(u: Vector<3>, v: Vector<3>) -> Vector<3> {
        let [ux, uy, uz] = u.coordinates;
        let [vx, vy, vz] = v.coordinates;

        let x = ux * vz - uz * vy;
        let y = uz * vx - ux * vz;
        let z = ux * vy - uy * vx;

        Vector::new([x, y, z])
    }
}



impl<const N: usize> Add for Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Vector<N> {
        let mut coordinates = [0.0; N];

        for i in 0..N {
            coordinates[i] = self.coordinates[i] + other.coordinates[i];
        }

        Vector::new(coordinates)
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.coordinates[i] += rhs.coordinates[i];
        }

        self.norm_value.stale = true;
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, scalar: f64) -> Vector<N> {
        Vector::new_with_norm(self.coordinates.map(|c| c / scalar), self.norm_value.get_static() / scalar)
    }

}

impl<const N: usize> DivAssign<f64> for Vector<N> {
    fn div_assign(&mut self, scalar: f64) {
        for i in 0..N {
            self.coordinates[i] /= scalar;
        }

        let norm = self.norm();
        self.norm_value.set(norm / scalar);
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector::new_with_norm(self.coordinates.map(|c| c * scalar), self.norm_value.get_static() * scalar)
    }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = f64;

    fn mul(self, other: Vector<N>) -> f64 {
        let mut out = 0.0;

        for i in 0..N {
            out += self.coordinates[i] * other.coordinates[i];
        }

        out
    }
}

impl Mul<Quaternion> for Vector<3> {
    type Output = Vector<3>;

    fn mul(self, other: Quaternion) -> Vector<3> {
        let (r, i, j, k) = Quaternion::conj(
            (0.0, self.coordinates[0], self.coordinates[1], self.coordinates[2]),
            other.get()
        );

        Vector::new([i, j, k])
    }
}

impl<const N: usize> MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, scalar: f64) {
        for i in 0..N {
            self.coordinates[i] *= scalar;
        }

        let norm = self.norm();
        self.norm_value.set(norm * scalar);
    }
}

impl MulAssign<Quaternion> for Vector<3> {
    fn mul_assign(&mut self, other: Quaternion) {
        let (r, i, j, k) = Quaternion::conj(
            (0.0, self.coordinates[0], self.coordinates[1], self.coordinates[2]),
            other.get()
        );

        self.coordinates = [i, j, k];
        self.norm_value.stale = true;
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Vector<N> {
        Vector::new_with_norm(self.coordinates.map(|c| -c), self.norm_value.get_static())
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: Vector<N>) -> Vector<N> {
        let mut coordinates = [0.0; N];

        for i in 0..N {
            coordinates[i] = self.coordinates[i] - other.coordinates[i];
        }

        Vector::new(coordinates)
    }

}

impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, other: Vector<N>) {
        for i in 0..N {
            self.coordinates[i] -= other.coordinates[i];
        }

        self.norm_value.stale = true;
    }
}