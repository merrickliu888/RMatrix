use rmatrix::matrices::basic_matrix::*;
use rmatrix::Matrix;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn test_new() {
    let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let matrix = BasicMatrix::new(2, 2, data.clone());

    // We need to implement getters to properly test this
    // For now, we'll test indirectly through other methods
    let identity = BasicMatrix::identity(2);
    let result = matrix.matrix_multiplication(&identity);

    // Should be unchanged after multiplying by identity
    assert_eq!(*result.get_data(), data);
}

#[test]
fn test_zeroes() {
    let matrix = BasicMatrix::zeroes(2, 3);

    // Check dimensions
    assert_eq!(matrix.get_rows(), 2);
    assert_eq!(matrix.get_cols(), 3);

    // Check all elements are zero
    for row in matrix.get_data() {
        for &val in row {
            assert_eq!(val, 0.0);
        }
    }
}

#[test]
fn test_identity() {
    let matrix = BasicMatrix::identity(3);

    // Check dimensions
    assert_eq!(matrix.get_rows(), 3);
    assert_eq!(matrix.get_cols(), 3);

    // Check diagonal elements are 1 and others are 0
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                assert_eq!(matrix.get_data()[i][j], 1.0);
            } else {
                assert_eq!(matrix.get_data()[i][j], 0.0);
            }
        }
    }
}

#[test]
fn test_matrix_multiplication_identity() {
    let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let matrix = BasicMatrix::new(2, 3, data.clone());
    let identity = BasicMatrix::identity(3); // Identity matrix for multiplication

    let result = matrix.matrix_multiplication(&identity);

    // Should be unchanged after multiplying by identity
    assert_eq!(*result.get_data(), data);
}

#[test]
fn test_matrix_addition() {
    let matrix1 = BasicMatrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = BasicMatrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = matrix1.matrix_addition(&matrix2);

    // Check result
    assert_eq!(*result.get_data(), vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
}

#[test]
fn test_matrix_addition_dimension_mismatch() {
    let matrix1 = BasicMatrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = BasicMatrix::new(2, 3, vec![vec![5.0, 6.0, 7.0], vec![8.0, 9.0, 10.0]]);

    // Should panic with dimension mismatch
    let result = catch_unwind(AssertUnwindSafe(|| matrix1.matrix_addition(&matrix2)));

    assert!(result.is_err());
}

#[test]
fn test_matrix_subtraction() {
    let matrix1 = BasicMatrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let matrix2 = BasicMatrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let result = matrix1.matrix_subtraction(&matrix2);

    // Check result
    assert_eq!(*result.get_data(), vec![vec![4.0, 4.0], vec![4.0, 4.0]]);
}

#[test]
fn test_matrix_subtraction_dimension_mismatch() {
    let matrix1 = BasicMatrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = BasicMatrix::new(2, 3, vec![vec![5.0, 6.0, 7.0], vec![8.0, 9.0, 10.0]]);

    // Should panic with dimension mismatch
    let result = catch_unwind(AssertUnwindSafe(|| matrix1.matrix_subtraction(&matrix2)));

    assert!(result.is_err());
}

#[test]
fn test_matrix_multiplication() {
    let matrix1 = BasicMatrix::new(2, 3, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let matrix2 = BasicMatrix::new(
        3,
        2,
        vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]],
    );

    let result = matrix1.matrix_multiplication(&matrix2);

    // Check dimensions
    assert_eq!(result.get_rows(), 2);
    assert_eq!(result.get_cols(), 2);

    // Check result: [1*7+2*9+3*11, 1*8+2*10+3*12; 4*7+5*9+6*11, 4*8+5*10+6*12]
    assert_eq!(
        *result.get_data(),
        vec![vec![58.0, 64.0], vec![139.0, 154.0]]
    );
}

#[test]
fn test_matrix_multiplication_dimension_mismatch() {
    let matrix1 = BasicMatrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = BasicMatrix::new(3, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0], vec![9.0, 10.0]]);

    // Should panic with dimension mismatch
    let result = catch_unwind(AssertUnwindSafe(|| matrix1.matrix_multiplication(&matrix2)));

    assert!(result.is_err());
}

#[test]
fn test_scalar_multiplication() {
    let matrix = BasicMatrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let scalar = 2.0;

    let result = matrix.scalar_multiplication(&scalar);

    // Check result
    assert_eq!(*result.get_data(), vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
}

#[test]
fn test_transpose() {
    let mut matrix = BasicMatrix::new(2, 3, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

    matrix.transpose();

    // Check dimensions are swapped
    assert_eq!(matrix.get_rows(), 3);
    assert_eq!(matrix.get_cols(), 2);

    // Check data is transposed
    assert_eq!(
        *matrix.get_data(),
        vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]
    );
}
