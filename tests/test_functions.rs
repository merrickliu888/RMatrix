use rmatrix::Matrix;

pub fn test_new<T: Matrix>() {
    let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let matrix = T::new(data.clone());

    assert_eq!(matrix.shape(), (2, 2));
    assert_eq!(matrix.num_rows(), 2);
    assert_eq!(matrix.num_cols(), 2);
    assert_eq!(matrix.get_data(), data);
}

pub fn test_zeroes<T: Matrix>() {
    let matrix = T::zeroes(2, 3);

    // Check dimensions
    assert_eq!(matrix.num_rows(), 2);
    assert_eq!(matrix.num_cols(), 3);

    // Check all elements are zero
    for row in matrix.get_data() {
        for val in row {
            assert_eq!(val, 0.0);
        }
    }
}

pub fn test_identity<T: Matrix>() {
    let matrix = T::identity(3);

    // Check dimensions
    assert_eq!(matrix.num_rows(), 3);
    assert_eq!(matrix.num_cols(), 3);

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

pub fn test_matrix_multiplication_identity<T: Matrix>() {
    let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let matrix = T::new(data.clone());
    let identity = T::identity(3); // Identity matrix for multiplication

    let result = matrix.matrix_multiplication(&identity);

    // Should be unchanged after multiplying by identity
    assert_eq!(result.get_data(), data);
}

pub fn test_matrix_addition<T: Matrix>() {
    let matrix1 = T::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = T::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = matrix1.matrix_addition(&matrix2);

    // Check result
    assert_eq!(result.get_data(), vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
}

pub fn test_matrix_subtraction<T: Matrix>() {
    let matrix1 = T::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let matrix2 = T::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let result = matrix1.matrix_subtraction(&matrix2);

    // Check result
    assert_eq!(result.get_data(), vec![vec![4.0, 4.0], vec![4.0, 4.0]]);
}

pub fn test_matrix_multiplication<T: Matrix>() {
    let matrix1 = T::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let matrix2 = T::new(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);

    let result = matrix1.matrix_multiplication(&matrix2);

    // Check dimensions
    assert_eq!(result.num_rows(), 2);
    assert_eq!(result.num_cols(), 2);

    // Check result: [1*7+2*9+3*11, 1*8+2*10+3*12; 4*7+5*9+6*11, 4*8+5*10+6*12]
    assert_eq!(
        result.get_data(),
        vec![vec![58.0, 64.0], vec![139.0, 154.0]]
    );
}

pub fn test_scalar_multiplication<T: Matrix>() {
    let matrix = T::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let scalar = 2.0;

    let result = matrix.scalar_multiplication(scalar);

    // Check result
    assert_eq!(result.get_data(), vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
}
