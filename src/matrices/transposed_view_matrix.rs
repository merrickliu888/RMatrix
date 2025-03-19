use crate::Matrix;

/// Representing matrix as a one-dimensional vector
#[derive(Debug)]
pub struct TransposedViewMatrix {
    data: Vec<f64>,
    transposed_data: Vec<f64>,
    transposed: bool,
    shape: (usize, usize),
}

impl TransposedViewMatrix {
    fn new_from_vec(data: Vec<f64>, rows: usize, cols: usize, eager: bool) -> Self {
        let mut transposed_data = vec![0.0; data.len()];
        let mut transposed = false;

        // If eager, we will transpose the data immediately.
        if eager {
            for i in 0..rows {
                for j in 0..cols {
                    transposed_data[j * rows + i] = data[i * cols + j];
                }
            }
            transposed = true;
        }

        Self {
            data,
            transposed_data,
            transposed,
            shape: (rows, cols),
        }
    }

    pub fn get_data_vec(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn get_transposed_data_vec(&self) -> &Vec<f64> {
        &self.transposed_data
    }

    #[inline(always)]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.num_cols() + col]
    }

    #[inline(always)]
    pub fn get_transposed(&self, row: usize, col: usize) -> f64 {
        self.transposed_data[row * self.num_rows() + col]
    }
}

impl Matrix for TransposedViewMatrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let data = data.into_iter().flatten().collect();
        Self::new_from_vec(data, rows, cols, true)
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
        self.data
            .chunks(self.num_cols())
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            transposed_data: vec![0.0; rows * cols],
            transposed: true,
            shape: (rows, cols),
        }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![0.0; size * size];

        for i in 0..size {
            data[i * size + i] = 1.0;
        }

        TransposedViewMatrix::new_from_vec(data, size, size, true)
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self::new_from_vec(data, self.num_rows(), self.num_cols(), false)
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self::new_from_vec(data, self.num_rows(), self.num_cols(), false)
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        if !other.transposed {
            panic!("Other matrix must be transposed");
        }

        let self_rows = self.num_rows();
        let self_cols = self.num_cols();
        let other_cols = other.num_cols();

        let mut res = vec![0.0; self_rows * other_cols];

        for i in 0..self_rows {
            for j in 0..other_cols {
                for k in 0..self_cols {
                    // Expects other to be transposed
                    res[i * other_cols + j] += self.get(i, k) * other.get_transposed(j, k);
                }
            }
        }

        Self::new_from_vec(res, self_rows, other_cols, false)
    }

    fn scalar_multiplication(&self, scalar: f64) -> Self {
        let data = self.data.iter().map(|a| a * scalar).collect();
        Self::new_from_vec(data, self.num_rows(), self.num_cols(), false)
    }
}
