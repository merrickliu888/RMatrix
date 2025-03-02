use crate::matrices::benchmarking::{BenchmarkData, BenchmarkDatas};
use rmatrix::matrices::basic_matrix::BasicMatrix;

pub fn benchmark_basic_matrix(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> BenchmarkDatas {
    let matrices = convert_to_basic_matrices(matrix_vectors);

    let matrix_addition_data = benchmark_matrix_addition(&matrices);
    let matrix_subtraction_data = benchmark_matrix_subtraction(&matrices);
    let matrix_multiplication_data = benchmark_matrix_multiplication(&matrices);
    let scalar_multiplication_data = benchmark_scalar_multiplication(&matrices);

    BenchmarkDatas::new(
        matrix_addition_data,
        matrix_subtraction_data,
        matrix_multiplication_data,
        scalar_multiplication_data,
    )
}

fn convert_to_basic_matrices(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> Vec<BasicMatrix> {
    Vec::new()
}

fn benchmark_matrix_addition(matrices: &Vec<BasicMatrix>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_matrix_subtraction(matrices: &Vec<BasicMatrix>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_matrix_multiplication(matrices: &Vec<BasicMatrix>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_scalar_multiplication(matrices: &Vec<BasicMatrix>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}
