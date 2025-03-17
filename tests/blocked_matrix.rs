use rmatrix::matrices::blocked_matrix::*;
mod test_functions;
use test_functions::*;

#[test]
fn blocked_matrix_test_new() {
    test_new::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_zeroes() {
    test_zeroes::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_identity() {
    test_identity::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_matrix_addition() {
    test_matrix_addition::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<BlockedMatrix>();
}

#[test]
fn blocked_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<BlockedMatrix>();
}
