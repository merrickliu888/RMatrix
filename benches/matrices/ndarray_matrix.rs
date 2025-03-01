use crate::matrices::benchmark_data::{BenchmarkData, BenchmarkDatas};
use ndarray::Array2;

pub fn benchmark_ndarray_matrix(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> BenchmarkDatas {
    let matrices = convert_to_ndarray_matrices(matrix_vectors);

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

fn convert_to_ndarray_matrices(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> Vec<Array2<f64>> {
    Vec::new()
}

fn benchmark_matrix_addition(matrices: &Vec<Array2<f64>>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_matrix_subtraction(matrices: &Vec<Array2<f64>>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_matrix_multiplication(matrices: &Vec<Array2<f64>>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}

fn benchmark_scalar_multiplication(matrices: &Vec<Array2<f64>>) -> BenchmarkData {
    BenchmarkData::new(Vec::new(), Vec::new())
}
