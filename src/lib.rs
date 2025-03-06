pub trait Matrix {
    type Implementation: Matrix;

    fn zeroes(rows: usize, cols: usize) -> Self;

    fn identity(size: usize) -> Self;

    fn matrix_addition(&self, other: &Self) -> Self;

    fn matrix_subtraction(&self, other: &Self) -> Self;

    fn matrix_multiplication(&self, other: &Self) -> Self;

    fn scalar_multiplication(&self, scalar: f64) -> Self;
}

pub mod matrices;
