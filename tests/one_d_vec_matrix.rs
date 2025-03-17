use rmatrix::matrices::one_d_vec_matrix::*;
mod test_functions;
use test_functions::*;
#[test]
fn one_d_vec_matrix_test_new() {
    test_new::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_zeroes() {
    test_zeroes::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_identity() {
    test_identity::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_matrix_multiplication_identity() {
    test_matrix_multiplication_identity::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_matrix_addition() {
    test_matrix_addition::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_matrix_subtraction() {
    test_matrix_subtraction::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_matrix_multiplication() {
    test_matrix_multiplication::<OneDVecMatrix>();
}

#[test]
fn one_d_vec_matrix_test_scalar_multiplication() {
    test_scalar_multiplication::<OneDVecMatrix>();
}
