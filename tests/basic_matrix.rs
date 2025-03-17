use rmatrix::matrices::basic_matrix::*;
use test_functions::*;
mod test_functions;

#[test]
fn basic_matrix_test_new() {
    test_new::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_zeroes() {
    test_zeroes::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_identity() {
    test_identity::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_matrix_addition() {
    test_matrix_addition::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<BasicMatrix>();
}

#[test]
fn basic_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<BasicMatrix>();
}
