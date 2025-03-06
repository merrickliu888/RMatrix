use rmatrix::Matrix;
use rmatrix::matrices::basic_matrix::BasicMatrix;

fn main() {
    let matrix1 = BasicMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = BasicMatrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    // Addition
    let matrix_sum = matrix1.matrix_addition(&matrix2);

    // Subtraction
    let matrix_difference = matrix1.matrix_subtraction(&matrix2);

    // Multiplication
    let matrix_product = matrix1.matrix_multiplication(&matrix2);

    // Scalar multiplication
    let matrix_scalar_product = matrix1.scalar_multiplication(2.0);

    println!("{:?}", matrix_sum);
    println!("{:?}", matrix_difference);
    println!("{:?}", matrix_product);
    println!("{:?}", matrix_scalar_product);
    println!("{:?}", matrix1);
}
