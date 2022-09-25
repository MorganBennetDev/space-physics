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



mod matrix_view;



type MatrixView = matrix_view::MatrixView;



const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

const fn min(a: usize, b: usize) -> usize {
    [a, b][(a > b) as usize]
}



pub struct Matrix<const N: usize, const M: usize> {
    view: MatrixView
}



impl<const N: usize, const M: usize> Matrix<N, M> {
    const PADDING_EXPONENT: usize = (usize::BITS - usize::leading_zeros(max(M, N))) as usize;

    pub fn new(data: Vec<Vec<f64>>) -> Matrix<N, M> {
        Matrix { view: MatrixView::new(data, Matrix::<N, M>::PADDING_EXPONENT) }
    }

    fn new_from_view(data: MatrixView) -> Matrix<N, M> {
        Matrix { view: data }
    }
}



impl<const N: usize, const M: usize> Add for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn add(self, other: Matrix<N, M>) -> Matrix<N, M> {
        Matrix::new_from_view(self.view + other.view)
    }
}



impl<const N: usize, const M: usize> AddAssign for Matrix<N, M> {
    fn add_assign(&mut self, rhs: Self) {
        self.view += rhs.view;
    }
}



impl<const N: usize, const M: usize> Div<f64> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn div(self, scalar: f64) -> Matrix<N, M> {
        let out = Matrix::<N, M> {
            view: MatrixView::new(vec![vec![0.0; M]; N], Matrix::<N, M>::PADDING_EXPONENT)
        };

        for i in 0..N {
            for j in 0..M {
                out.view.data[i][j] = self.view.data[i][j] / scalar;
            }
        }

        out
    }
}



impl<const N: usize, const M: usize> DivAssign<f64> for Matrix<N, M> {
    fn div_assign(&mut self, scalar: f64) {
        for i in 0..N {
            for j in 0..M {
                self.view.data[i][j] /= scalar;
            }
        }
    }
}



impl<const N: usize, const M: usize> Mul<f64> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn mul(self, scalar: f64) -> Matrix<N, M> {
        let out = Matrix::<N, M> {
            view: MatrixView::new(vec![vec![0.0; M]; N], Matrix::<N, M>::PADDING_EXPONENT)
        };

        for i in 0..N {
            for j in 0..M {
                out.view.data[i][j] = self.view.data[i][j] * scalar;
            }
        }

        out
    }
}



impl<const N: usize, const M: usize> MulAssign<f64> for Matrix<N, M> {
    fn mul_assign(&mut self, scalar: f64) {
        for i in 0..N {
            for j in 0..M {
                self.view.data[i][j] *= scalar;
            }
        }
    }
}



impl<const N: usize, const M: usize, const P: usize> Mul<Matrix<M, P>> for Matrix<N, M> {
    type Output = Matrix<N, P>;

    fn mul(self, other: Matrix<M, P>) -> Matrix<N, P> {
        Matrix::new_from_view(self.view * other.view)
    }
}



impl<const N: usize, const M: usize> MulAssign<Matrix<M, M>> for Matrix<N, M> {
    fn mul_assign(&mut self, rhs: Matrix<M, M>) {
        self.view *= rhs.view;
    }
}



impl<const N: usize, const M: usize> Neg for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn neg(self) -> Matrix<N, M> {
        Matrix::new_from_view(-self.view)
    }
}



impl<const N: usize, const M: usize> Sub for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn sub(self, other: Matrix<N, M>) -> Matrix<N, M> {
        Matrix::new_from_view(self.view - other.view)
    }
}



impl<const N: usize, const M: usize> SubAssign for Matrix<N, M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.view -= rhs.view;
    }
}



impl<const N: usize, const M: usize> Index<(usize, usize)> for Matrix<N, M> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &f64 {
        if index.0 >= N || index.1 >= M {
            panic!("Index out of bounds");
        }

        &self.view[index]
    }
}



impl<const N: usize, const M: usize> IndexMut<(usize, usize)> for Matrix<N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64 {
        if index.0 >= N || index.1 >= M {
            panic!("Index out of bounds");
        }

        &mut self.view[index]
    }
}