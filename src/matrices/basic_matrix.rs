use crate::Matrix;

/// This is a naive implementation of a matrix with no optimizations.
#[derive(Debug)]
pub struct BasicMatrix {
    data: Vec<Vec<f64>>,
    shape: (usize, usize),
}

impl Matrix for BasicMatrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let shape = (data.len(), data[0].len());
        Self { data, shape }
    }

    fn shape(&self) -> (usize, usize) {
        self.shape
    }

    fn num_rows(&self) -> usize {
        self.shape.0
    }

    fn num_cols(&self) -> usize {
        self.shape.1
    }

    fn get_data(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }

    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
            shape: (rows, cols),
        }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Self {
            data,
            shape: (size, size),
        }
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        let self_rows = self.num_rows();
        let self_cols = self.num_cols();

        let mut res = Self::zeroes(self_rows, self_cols);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j]
            }
        }

        res
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        let self_rows = self.num_rows();
        let self_cols = self.num_cols();

        let mut res = Self::zeroes(self_rows, self_cols);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j]
            }
        }

        res
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        let self_rows = self.num_rows();
        let self_cols = self.num_cols();
        let other_cols = other.num_cols();

        let mut res = Self::zeroes(self_rows, other_cols);

        for i in 0..self_rows {
            for j in 0..other_cols {
                for k in 0..self_cols {
                    res.data[i][j] += self.data[i][k] * other.data[k][j]
                }
            }
        }

        res
    }

    fn scalar_multiplication(&self, scalar: f64) -> Self {
        let self_rows = self.num_rows();
        let self_cols = self.num_cols();

        let mut res = Self::zeroes(self_rows, self_cols);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[i][j] = self.data[i][j] * scalar;
            }
        }

        res
    }
}
