use crate::benchmarking::{BenchmarkData, BenchmarkResults};
use ndarray::Array2;

pub fn benchmark_ndarray_matrix(
    matrix_vectors1: &Vec<Vec<Vec<f64>>>,
    matrix_vectors2: &Vec<Vec<Vec<f64>>>,
) -> BenchmarkResults {
    let matrices1 = convert_to_ndarray_matrices(matrix_vectors1);
    let matrices2 = convert_to_ndarray_matrices(matrix_vectors2);

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

fn convert_to_ndarray_matrices(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> Vec<Array2<f64>> {
    let mut matrices = Vec::new();

    for matrix_vector in matrix_vectors.iter() {
        let shape = (matrix_vector.len(), matrix_vector[0].len());
        let arr = Array2::from_shape_vec(shape, matrix_vector.iter().flatten().cloned().collect())
            .expect("Failed to convert vector into Array2");
        matrices.push(arr)
    }

    matrices
}

fn benchmark_matrix_addition(
    matrices1: &Vec<Array2<f64>>,
    matrices2: &Vec<Array2<f64>>,
) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();
    BenchmarkData::new(sizes, times)
}

fn benchmark_matrix_subtraction(
    matrices1: &Vec<Array2<f64>>,
    matrices2: &Vec<Array2<f64>>,
) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();
    BenchmarkData::new(sizes, times)
}

fn benchmark_matrix_multiplication(
    matrices1: &Vec<Array2<f64>>,
    matrices2: &Vec<Array2<f64>>,
) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();
    BenchmarkData::new(sizes, times)
}

fn benchmark_scalar_multiplication(matrices1: &Vec<Array2<f64>>, scalar: f64) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();
    BenchmarkData::new(sizes, times)
}
