use rmatrix::matrices::multithread_matrix::*;
use test_functions::*;
mod test_functions;

#[test]
fn multithread_matrix_test_new() {
    test_new::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_zeroes() {
    test_zeroes::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_identity() {
    test_identity::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_matrix_addition() {
    test_matrix_addition::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<MultithreadMatrix>();
}

#[test]
fn multithread_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<MultithreadMatrix>();
}
