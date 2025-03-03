use crate::benchmarking::{BenchmarkData, BenchmarkResults};
use rmatrix::matrices::basic_matrix::BasicMatrix;

pub fn benchmark_basic_matrix(
    matrix_vectors1: &Vec<Vec<Vec<f64>>>,
    matrix_vectors2: &Vec<Vec<Vec<f64>>>,
) -> BenchmarkResults {
    let matrices1 = convert_to_basic_matrices(matrix_vectors1);
    let matrices2 = convert_to_basic_matrices(matrix_vectors2);

    let matrix_addition_data = benchmark_matrix_addition(&matrices1, &matrices2);
    let matrix_subtraction_data = benchmark_matrix_subtraction(&matrices1, &matrices2);
    let matrix_multiplication_data = benchmark_matrix_multiplication(&matrices1, &matrices2);
    let scalar_multiplication_data = benchmark_scalar_multiplication(&matrices1, 1.5);

    BenchmarkResults::new(
        matrix_addition_data,
        matrix_subtraction_data,
        matrix_multiplication_data,
        scalar_multiplication_data,
    )
}

fn convert_to_basic_matrices(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> Vec<BasicMatrix> {
    Vec::new()
}

fn benchmark_matrix_addition(
    matrices1: &Vec<BasicMatrix>,
    matrices2: &Vec<BasicMatrix>,
) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_matrix_subtraction(
    matrices1: &Vec<BasicMatrix>,
    matrices2: &Vec<BasicMatrix>,
) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_matrix_multiplication(
    matrices1: &Vec<BasicMatrix>,
    matrices2: &Vec<BasicMatrix>,
) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_scalar_multiplication(matrices1: &Vec<BasicMatrix>, scalar: f64) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}
