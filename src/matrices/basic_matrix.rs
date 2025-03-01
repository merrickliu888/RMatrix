use crate::Matrix;

pub struct BasicMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl BasicMatrix {
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        Self { rows, cols, data }
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
}

impl Matrix for BasicMatrix {
    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
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
        }
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        if self.rows != other.rows || self.cols != other.cols {
            panic!(
                "Add: Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
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
                "Sub: Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
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
        if self.cols != other.rows {
            panic!(
                "Mult: Dimension mismatch - LHS ({}x{}), RHS ({}x{})",
                self.rows, self.cols, other.rows, other.cols
            )
        }

        let mut res = Self::zeroes(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    res.data[i][j] += self.data[i][k] * other.data[k][j]
                }
            }
        }

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
        let mut res = Self::zeroes(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        self.data = res.data;
        let temp = self.rows;
        self.rows = self.cols;
        self.cols = temp;
    }
}
