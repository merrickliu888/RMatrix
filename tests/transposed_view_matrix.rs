use rmatrix::matrices::transposed_view_matrix::*;
mod test_functions;
use test_functions::*;
#[test]
fn transposed_view_matrix_test_new() {
    test_new::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_zeroes() {
    test_zeroes::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_identity() {
    test_identity::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_matrix_addition() {
    test_matrix_addition::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<TransposedViewMatrix>();
}

#[test]
fn transposed_view_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<TransposedViewMatrix>();
}
