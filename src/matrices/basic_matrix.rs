use crate::Matrix;

pub struct BasicMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
    transposed: bool,
}

impl BasicMatrix {
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        Self {
            rows,
            cols,
            data,
            transposed: false,
        }
    }
}

impl Matrix for BasicMatrix {
    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
            transposed: false,
        }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Self {
            rows: size,
            cols: size,
            data,
            transposed: false,
        }
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        if self.rows != other.rows || self.cols != other.cols {
            panic!(
                "Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
                self.rows, self.cols, other.rows, other.cols
            );
        }

        let mut res = Self::zeroes(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j]
            }
        }

        res
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        if self.rows != other.rows || self.cols != other.cols {
            panic!(
                "Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
                self.rows, self.cols, other.rows, other.cols
            );
        }

        let mut res = Self::zeroes(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j]
            }
        }

        res
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        let mut res = Self::zeroes(self.rows, self.cols);

        // TODO

        res
    }

    fn scalar_multiplication(&self, scalar: &f64) -> Self {
        let mut res = Self::zeroes(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * scalar;
            }
        }

        res
    }

    fn transpose(&mut self) -> () {
        self.transposed = !self.transposed;
    }
}
