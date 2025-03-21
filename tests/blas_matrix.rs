use rmatrix::matrices::blas_matrix::*;
use test_functions::*;
mod test_functions;

#[test]
fn blas_matrix_test_new() {
    test_new::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_zeroes() {
    test_zeroes::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_identity() {
    test_identity::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_matrix_addition() {
    test_matrix_addition::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<BlasMatrix>();
}

#[test]
fn blas_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<BlasMatrix>();
}
