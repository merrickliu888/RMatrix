use crate::Matrix;

/// Representing matrix as a one-dimensional vector
#[derive(Debug)]
pub struct OneDVecMatrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl OneDVecMatrix {
    pub fn new_from_vec(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self { data, rows, cols }
    }

    pub fn new_from_vec_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let data = data.into_iter().flatten().collect();
        Self::new_from_vec(data, rows, cols)
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_data(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn get_data_vec_vec(&self) -> Vec<Vec<f64>> {
        self.data
            .chunks(self.cols)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

impl Matrix for OneDVecMatrix {
    type Implementation = Self;

    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![0.0; size * size];

        for i in 0..size {
            data[i * size + i] = 1.0;
        }

        OneDVecMatrix::new_from_vec(data, size, size)
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self::new_from_vec(data, self.rows, self.cols)
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self::new_from_vec(data, self.rows, self.cols)
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        let mut res = vec![0.0; self.rows * other.cols];

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    res[i * other.cols + j] +=
                        self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
            }
        }

        Self::new_from_vec(res, self.rows, other.cols)
    }

    fn scalar_multiplication(&self, scalar: f64) -> Self {
        let data = self.data.iter().map(|a| a * scalar).collect();
        Self::new_from_vec(data, self.rows, self.cols)
    }
}
