use std::ops::{
    Add,
    AddAssign,
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



const SW_THRESHOLD_EXPONENT: usize = 4;



pub struct MatrixView {
    pub data: Box<Vec<Vec<f64>>>,
    block: (usize, usize),
    block_exponent: usize
}



impl MatrixView {
    pub fn new(data: Vec<Vec<f64>>, block_exponent: usize) -> Self {
        Self {
            data: Box::new(data),
            block: (0, 0),
            block_exponent
        }
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn block(&self, i: usize, j: usize) -> MatrixView {
        if self.block_exponent == 0 {
            return Self {
                data: self.data,
                block: (0, 0),
                block_exponent: 0
            }
        } else {
            return Self {
                data: self.data,
                block: (i, j),
                block_exponent: self.block_exponent - 1
            }
        }
    }

    fn set(&mut self, other: &MatrixView) {
        if self.block_exponent != other.block_exponent {
            panic!("Matrices must be of the same size (set)");
        }

        let start_row_self = self.block.0 << self.block_exponent;
        let start_col_self = self.block.1 << self.block_exponent;
        let start_row_other = other.block.0 << other.block_exponent;
        let start_col_other = other.block.1 << other.block_exponent;

        let end_row_self = (start_row_self + 1 << self.block_exponent).min(self.rows());
        let end_col_self = (start_col_self + 1 << self.block_exponent).min(self.cols());
        let end_row_other = (start_row_other + 1 << other.block_exponent).min(other.rows());
        let end_col_other = (start_col_other + 1 << other.block_exponent).min(other.cols());

        let other_rows = end_row_other - start_row_other;
        let other_cols = end_col_other - start_col_other;

        if start_col_self + other_cols > end_col_self {
            for row in self.data.iter_mut() {
                row.resize(start_col_self + other_cols, 0.0);
            }
        }

        if start_row_self + other_rows > end_row_self {
            self.data.resize(start_row_self + other_rows, vec![0.0; self.cols()]);
        }

        for i in 0..other_cols {
            for j in 0..other_rows {
                self.data[start_row_self + j][start_col_self + i] = other.data[start_row_other + j][start_col_other + i];
            }
        }

        for i in other_rows..(end_row_self - start_row_self) {
            for j in 0..(end_col_self - start_col_self) {
                self.data[start_row_self + i][start_col_self + j] = 0.0;
            }
        }

        for i in 0..other_rows {
            for j in other_cols..(end_col_self - start_col_self) {
                self.data[start_row_self + i][start_col_self + j] = 0.0;
            }
        }
    }

    fn addition<const scalar: i32>(out: &mut Self, rhs: &Self) {
        if out.block_exponent != rhs.block_exponent {
            panic!("Size of blocks must be equal (addition)");
        }

        let start_row_out = out.block.0 << out.block_exponent;
        let start_col_out = out.block.1 << out.block_exponent;
        let start_row_rhs = rhs.block.0 << rhs.block_exponent;
        let start_col_rhs = rhs.block.1 << rhs.block_exponent;
        let end_row = out.rows().max(rhs.rows()).min(start_row_out + 1 << out.block_exponent);
        let end_col = out.cols().max(rhs.cols()).min(start_col_out + 1 << out.block_exponent);

        if end_col > out.cols() {
            for row in out.data.iter_mut() {
                row.resize(end_col, 0.0);
            }
        }

        if end_row > out.rows() {
            out.data.resize(end_row, vec![0.0; end_col]);
        }

        let nonempty_rows = end_row - start_row_out.max(start_row_rhs);
        let nonempty_cols = end_col - start_col_out.max(start_col_rhs);

        for i in 0..nonempty_rows {
            for j in 0..nonempty_cols {
                out.data[start_row_out + i][start_col_out + j] += rhs.data[start_row_rhs + i][start_col_rhs + j] * (scalar as f64);
            }
        }
    }

    fn multiplication_schoolbook_inplace(M: &mut Self, N: &Self) {
        if M.block_exponent != N.block_exponent {
            panic!("Size of blocks must be equal (multiplication)");
        }

        let start_row_M = M.block.0 << M.block_exponent;
        let start_col_M = M.block.1 << M.block_exponent;
        let start_row_N = N.block.0 << N.block_exponent;
        let start_col_N = N.block.1 << N.block_exponent;

        let dot_end = N.rows().min(M.cols()).min(start_row_M + 1 << M.block_exponent);
        let dot_len = dot_end - start_row_M.max(start_row_N);
        
        let end_row = M.rows().min(start_row_M + 1 << M.block_exponent);
        let end_col = N.cols().min(start_col_M + 1 << M.block_exponent);

        let nonempty_rows = end_row - start_row_M.max(start_row_N);
        let nonempty_cols = end_col - start_col_M.max(start_col_N);

        for i in 0..nonempty_rows {
            let row = M.data[start_row_M + i].clone();
            let row_index = start_row_M + i;

            for j in 0..nonempty_cols {
                let mut sum = 0.0;
                let dot_col_index = start_col_N + j;

                for k in 0..dot_len {
                    sum += row[start_col_M + k] * N.data[start_row_N + k][dot_col_index];
                }

                M.data[row_index][start_col_M + j] = sum;
            }
        }
    }

    fn multiplication_strassen_winograd(M: &Self, N: &Self) -> Self {
        if M.block_exponent != N.block_exponent {
            panic!("Size of blocks must be equal (multiplication)");
        }
        
        let start_row_M = M.block.0 << M.block_exponent;
        let start_col_M = M.block.1 << M.block_exponent;
        let start_row_N = N.block.0 << N.block_exponent;
        let start_col_N = N.block.1 << N.block_exponent;

        let end_row_M = M.rows().min(start_row_M + 1 << M.block_exponent);
        let end_col_M = M.cols().min(start_col_M + 1 << M.block_exponent);
        let end_row_N = N.rows().min(start_row_N + 1 << N.block_exponent);
        let end_col_N = N.cols().min(start_col_N + 1 << N.block_exponent);

        let m = end_row_M - start_row_M;
        let p = end_col_N - start_col_N;
        
        let mut out = Self::new(vec![vec![0.0; p]; m], M.block_exponent);

        let out11 = out.block(0, 0);
        let out12 = out.block(0, 1);
        let out21 = out.block(1, 0);
        let out22 = out.block(1, 1);

        let a = M.block(0, 0);
        let b = M.block(0, 1);
        let c = M.block(1, 0);
        let d = M.block(1, 1);

        let A = N.block(0, 0);
        let B = N.block(1, 0);
        let C = N.block(0, 1);
        let D = N.block(1, 1);

        out11.set(&(a * A));
        out21.set(&(C - D));
        
        let z = a - c;

        out12.set(&(z - d));

        let w = out11 + (out12 * out21);

        out11 += b * B;

        out12 += b;
        out12 *= D;

        out22 = (c + d) * (C - A);

        out12 += w + out22;

        w -= z * out21;

        out21 = B - A;
        out21 *= d;
        out21 += w;

        out22 += w;

        out
    }
}



impl Add for MatrixView {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = MatrixView::new(Vec::<Vec::<f64>>::new(), self.block_exponent);

        MatrixView::addition::<1>(&mut out, &rhs);

        out
    }
}



impl AddAssign for MatrixView {
    fn add_assign(&mut self, rhs: Self) {
        MatrixView::addition::<1>(self, &rhs);
    }
}



impl Sub for MatrixView {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = MatrixView::new(Vec::<Vec::<f64>>::new(), self.block_exponent);

        MatrixView::addition::<-1>(&mut out, &rhs);

        out
    }
}



impl SubAssign for MatrixView {
    fn sub_assign(&mut self, rhs: Self) {
        MatrixView::addition::<-1>(self, &rhs);
    }
}



impl Mul for MatrixView {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.block_exponent >= SW_THRESHOLD_EXPONENT {
            MatrixView::multiplication_strassen_winograd(&self, &rhs)
        } else {
            let mut out = MatrixView::new(Vec::<Vec::<f64>>::new(), self.block_exponent);

            MatrixView::multiplication_schoolbook_inplace(&mut out, &rhs);

            out
        }
    }
}



impl MulAssign for MatrixView {
    fn mul_assign(&mut self, rhs: Self) {
        if self.block_exponent >= SW_THRESHOLD_EXPONENT {
            self.set(&MatrixView::multiplication_strassen_winograd(self, &rhs));
        } else {
            MatrixView::multiplication_schoolbook_inplace(self, &rhs);
        }
    }
}



impl Neg for MatrixView {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let start_row = self.block.0 << self.block_exponent;
        let start_col = self.block.1 << self.block_exponent;
        let end_row = self.rows().min(start_row + 1 << self.block_exponent);
        let end_col = self.cols().min(start_col + 1 << self.block_exponent);

        MatrixView {
            data: Box::new(
                *(&self.data[start_row..end_row]
                    .iter()
                    .map(|row| row[start_col..end_col].iter().map(|x| -x).collect())
                    .collect())
            ),
            block: self.block,
            block_exponent: self.block_exponent
        }
    }
}



impl PartialEq for MatrixView {
    fn eq(&self, rhs: &Self) -> bool {
        if self.block_exponent != rhs.block_exponent {
            return false;
        }

        let start_row = self.block.0 << self.block_exponent;
        let start_col = self.block.1 << self.block_exponent;
        let end_row = self.rows().min(rhs.rows()).min(start_row + 1 << self.block_exponent);
        let end_col = self.cols().min(rhs.cols()).min(start_col + 1 << self.block_exponent);

        for i in start_row..end_row {
            for j in start_col..end_col {
                if self.data[i][j] != rhs.data[i][j] {
                    return false;
                }
            }
        }

        let max_row = self.rows().max(rhs.rows()).min(start_row + 1 << self.block_exponent);
        let max_col = self.cols().max(rhs.cols()).min(start_col + 1 << self.block_exponent);

        for i in end_row..max_row {
            for j in start_col..end_col {
                if self.data[i][j] != 0.0 || rhs.data[i][j] != 0.0 {
                    return false;
                }
            }
        }

        for i in start_row..end_row {
            for j in end_col..max_col {
                if self.data[i][j] != 0.0 || rhs.data[i][j] != 0.0 {
                    return false;
                }
            }
        }

        true
    }
}



impl Eq for MatrixView {}



impl Index<(usize, usize)> for MatrixView {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= 1 << self.block_exponent || index.1 >= 1 << self.block_exponent {
            return &0.0
        }
        
        let row = index.0 + self.block.0 << self.block_exponent;
        let col = index.1 + self.block.1 << self.block_exponent;

        if row >= self.rows() || col >= self.cols() {
            return &0.0
        }

        &self.data[row][col]
    }
}



impl IndexMut<(usize, usize)> for MatrixView {fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= 1 << self.block_exponent || index.1 >= 1 << self.block_exponent {
            return &mut 0.0
        }
        
        let row = index.0 + self.block.0 << self.block_exponent;
        let col = index.1 + self.block.1 << self.block_exponent;

        if row >= self.rows() || col >= self.cols() {
            return &mut 0.0
        }

        &mut self.data[row][col]
    }
}