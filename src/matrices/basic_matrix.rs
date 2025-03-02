use crate::Matrix;

// This is a naive implementation of a matrix with no optimizations.
#[derive(Debug)]
pub struct BasicMatrix {
    data: Vec<Vec<f64>>,
}

impl BasicMatrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    pub fn get_rows(&self) -> usize {
        self.data.len()
    }

    pub fn get_cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
}

impl Matrix for BasicMatrix {
    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
        }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Self { data }
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        let self_rows = self.get_rows();
        let self_cols = self.get_cols();
        let other_rows = other.get_rows();
        let other_cols = other.get_cols();

        if self_rows != other_rows || self_cols != other_cols {
            panic!(
                "Add: Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
                self_rows, self_cols, other_rows, other_cols
            );
        }

        let mut res = Self::zeroes(self_rows, self_cols);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j]
            }
        }

        res
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        let self_rows = self.get_rows();
        let self_cols = self.get_cols();
        let other_rows = other.get_rows();
        let other_cols = other.get_cols();

        if self_rows != other_rows || self_cols != other_cols {
            panic!(
                "Sub: Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
                self_rows, self_cols, other_rows, other_cols
            );
        }

        let mut res = Self::zeroes(self_rows, self_cols);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j]
            }
        }

        res
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        let self_rows = self.get_rows();
        let self_cols = self.get_cols();
        let other_rows = other.get_rows();
        let other_cols = other.get_cols();

        if self_cols != other_rows {
            panic!(
                "Mult: Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
                self_rows, self_cols, other_rows, other_cols
            )
        }

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

    fn scalar_multiplication(&self, scalar: &f64) -> Self {
        let self_rows = self.get_rows();
        let self_cols = self.get_cols();

        let mut res = Self::zeroes(self_rows, self_cols);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[i][j] = self.data[i][j] * scalar;
            }
        }

        res
    }

    fn transpose(&mut self) -> () {
        let self_rows = self.get_rows();
        let self_cols = self.get_cols();

        let mut res = Self::zeroes(self_cols, self_rows);

        for i in 0..self_rows {
            for j in 0..self_cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        self.data = res.data;
    }
}
