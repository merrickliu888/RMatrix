use rmatrix::matrices::ndarray_matrix::*;
use test_functions::*;
mod test_functions;

#[test]
fn ndarray_matrix_test_new() {
    test_new::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_zeroes() {
    test_zeroes::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_identity() {
    test_identity::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_matrix_addition() {
    test_matrix_addition::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<NdarrayMatrix>();
}

#[test]
fn ndarray_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<NdarrayMatrix>();
}
