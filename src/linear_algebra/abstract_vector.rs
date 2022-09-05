use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,

    Index,
    IndexMut
};

use std::cmp::{
    Eq,
    PartialEq
};



pub struct AbstractVector<const N: usize>([f64; N]);



impl<const N: usize> AbstractVector<N> {
    pub const ZERO: AbstractVector<N> = AbstractVector([0.0; N]);

    pub fn new(coordinates: [f64; N]) -> AbstractVector<N> {
        AbstractVector(coordinates)
    }

    pub fn norm(&self) -> f64 {
        AbstractVector::<N>::norm_static(&self.0)
    }

    pub fn norm_static<T: Index<usize, Output = f64>>(coordinates: &T) -> f64 {
        let mut norm = 0.0;

        for i in 0..N {
            norm += coordinates[i] * coordinates[i];
        }

        norm.sqrt()
    }

    pub fn entries(&self) -> &[f64; N] {
        &self.0
    }
}



impl<const N: usize> Add for AbstractVector<N> {
    type Output = AbstractVector<N>;

    fn add(self, other: AbstractVector<N>) -> AbstractVector<N> {
        let mut out = AbstractVector([0.0; N]);
        for i in 0..N {
            out.0[i] = self.0[i] + other.0[i];
        }

        out
    }
}

impl<const N: usize> AddAssign for AbstractVector<N> {
    fn add_assign(&mut self, other: AbstractVector<N>) {
        for i in 0..N {
            self.0[i] += other.0[i];
        }
    }
}

impl<const N: usize> Div<f64> for AbstractVector<N> {
    type Output = AbstractVector<N>;

    fn div(self, other: f64) -> AbstractVector<N> {
        let mut out = AbstractVector([0.0; N]);
        for i in 0..N {
            out.0[i] = self.0[i] / other;
        }

        out
    }
}

impl<const N: usize> DivAssign<f64> for AbstractVector<N> {
    fn div_assign(&mut self, other: f64) {
        for i in 0..N {
            self.0[i] /= other;
        }
    }
}

impl<const N: usize> Mul<f64> for AbstractVector<N> {
    type Output = AbstractVector<N>;

    fn mul(self, other: f64) -> AbstractVector<N> {
        let mut out = AbstractVector([0.0; N]);
        for i in 0..N {
            out.0[i] = self.0[i] * other;
        }

        out
    }
}

impl<const N: usize> MulAssign<f64> for AbstractVector<N> {
    fn mul_assign(&mut self, other: f64) {
        for i in 0..N {
            self.0[i] *= other;
        }
    }
}

impl<const N: usize> Mul<AbstractVector<N>> for AbstractVector<N> {
    type Output = f64;

    fn mul(self, other: AbstractVector<N>) -> f64 {
        let mut out = 0.0;
        for i in 0..N {
            out += self.0[i] * other.0[i];
        }

        out
    }
}

impl<const N: usize> Neg for AbstractVector<N> {
    type Output = AbstractVector<N>;

    fn neg(self) -> AbstractVector<N> {
        let mut out = AbstractVector([0.0; N]);
        for i in 0..N {
            out.0[i] = -self.0[i];
        }

        out
    }
}

impl<const N: usize> Sub for AbstractVector<N> {
    type Output = AbstractVector<N>;

    fn sub(self, other: AbstractVector<N>) -> AbstractVector<N> {
        let mut out = AbstractVector([0.0; N]);
        for i in 0..N {
            out.0[i] = self.0[i] - other.0[i];
        }

        out
    }
}

impl<const N: usize> SubAssign for AbstractVector<N> {
    fn sub_assign(&mut self, other: AbstractVector<N>) {
        for i in 0..N {
            self.0[i] -= other.0[i];
        }
    }
}



impl<const N: usize> Index<usize> for AbstractVector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for AbstractVector<N> {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.0[index]
    }
}



impl<const N: usize> Eq for AbstractVector<N> {}

impl<const N: usize> PartialEq for AbstractVector<N> {
    fn eq(&self, other: &AbstractVector<N>) -> bool {
        for i in 0..N {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}



impl<const N: usize> Copy for AbstractVector<N> {}

impl<const N: usize> Clone for AbstractVector<N> {
    fn clone(&self) -> AbstractVector<N> {
        AbstractVector(self.0)
    }
}