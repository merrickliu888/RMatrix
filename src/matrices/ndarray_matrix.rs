use crate::Matrix;
use ndarray::Array2;

/// Wrapper around ndarray::Array2
#[derive(Debug)]
pub struct NdarrayMatrix {
    data: Array2<f64>,
}

impl NdarrayMatrix {
    pub fn get_data(&self) -> &Array2<f64> {
        &self.data
    }
}

impl Matrix for NdarrayMatrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let shape = (data.len(), data[0].len());
        let converted_data =
            Array2::from_shape_vec(shape, data.iter().flatten().cloned().collect())
                .expect("Failed to convert vector into Array2");
        Self {
            data: converted_data,
        }
    }

    fn shape(&self) -> (usize, usize) {
        self.data.dim()
    }

    fn num_rows(&self) -> usize {
        self.data.nrows()
    }

    fn num_cols(&self) -> usize {
        self.data.ncols()
    }

    fn zeroes(rows: usize, cols: usize) -> Self {
        let shape = (rows, cols);
        let data = Array2::zeros(shape);
        Self { data }
    }

    fn identity(size: usize) -> Self {
        let data = Array2::eye(size);
        Self { data }
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        Self {
            data: &self.data + &other.data,
        }
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        Self {
            data: &self.data - &other.data,
        }
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        Self {
            data: &self.data * &other.data,
        }
    }

    fn scalar_multiplication(&self, scalar: f64) -> Self {
        Self {
            data: &self.data * scalar,
        }
    }
}
