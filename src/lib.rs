pub trait Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self;

    fn shape(&self) -> (usize, usize);

    fn num_rows(&self) -> usize;

    fn num_cols(&self) -> usize;

    fn zeroes(rows: usize, cols: usize) -> Self;

    fn identity(size: usize) -> Self;

    fn matrix_addition(&self, other: &Self) -> Self;

    fn matrix_subtraction(&self, other: &Self) -> Self;

    fn matrix_multiplication(&self, other: &Self) -> Self;

    fn scalar_multiplication(&self, scalar: f64) -> Self;
}

pub mod matrices;
