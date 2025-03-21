use rmatrix::Matrix;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

pub fn load_matrices(filename: &str) -> Vec<Vec<Vec<f64>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let matrices: Vec<Vec<Vec<f64>>> = serde_json::from_reader(reader).unwrap();
    matrices
}

pub fn benchmark_matrix<M: Matrix>(
    matrix_vectors1: &Vec<Vec<Vec<f64>>>,
    matrix_vectors2: &Vec<Vec<Vec<f64>>>,
) -> BenchmarkResults {
    let matrices1 = convert_to_matrices::<M>(matrix_vectors1);
    let matrices2 = convert_to_matrices::<M>(matrix_vectors2);

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

fn convert_to_matrices<M: Matrix>(matrix_vectors: &Vec<Vec<Vec<f64>>>) -> Vec<M> {
    matrix_vectors
        .iter()
        .map(|matrix| M::new(matrix.clone()))
        .collect()
}

fn benchmark_matrix_addition<M: Matrix>(matrices1: &Vec<M>, matrices2: &Vec<M>) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();

    for i in 0..matrices1.len() {
        sizes.push(matrices1[i].num_rows());

        let time = benchmark_function(
            || {
                let _ = matrices1[i].matrix_addition(&matrices2[i]);
            },
            10,
        );
        times.push(time);
    }

    BenchmarkData::new(sizes, times)
}

fn benchmark_matrix_subtraction<M: Matrix>(
    matrices1: &Vec<M>,
    matrices2: &Vec<M>,
) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();

    for i in 0..matrices1.len() {
        sizes.push(matrices1[i].num_rows());

        let time = benchmark_function(
            || {
                let _ = matrices1[i].matrix_subtraction(&matrices2[i]);
            },
            10,
        );
        times.push(time);
    }

    BenchmarkData::new(sizes, times)
}

fn benchmark_matrix_multiplication<M: Matrix>(
    matrices1: &Vec<M>,
    matrices2: &Vec<M>,
) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();

    for i in 0..matrices1.len() {
        sizes.push(matrices1[i].num_rows());

        let time = benchmark_function(
            || {
                let _ = matrices1[i].matrix_multiplication(&matrices2[i]);
            },
            10,
        );
        times.push(time);
    }

    BenchmarkData::new(sizes, times)
}

fn benchmark_scalar_multiplication<M: Matrix>(matrices1: &Vec<M>, scalar: f64) -> BenchmarkData {
    let mut sizes = Vec::new();
    let mut times = Vec::new();

    for i in 0..matrices1.len() {
        sizes.push(matrices1[i].num_rows());

        let time = benchmark_function(
            || {
                let _ = matrices1[i].scalar_multiplication(scalar);
            },
            10,
        );
        times.push(time);
    }

    BenchmarkData::new(sizes, times)
}

pub fn benchmark_function(func: impl Fn(), iter: usize) -> f64 {
    let mut total_time_elapsed: f64 = 0.0;
    for _ in 0..iter {
        let start = Instant::now();
        func();
        total_time_elapsed += start.elapsed().as_secs_f64();
    }
    total_time_elapsed / iter as f64
}

pub fn save_benchmark_results(filename: &str, results: &BenchmarkResults) {
    let file = File::create(filename).unwrap();
    serde_json::to_writer(file, results).unwrap();
}

#[derive(Serialize)]
pub struct BenchmarkResults {
    pub matrix_addition: BenchmarkData,
    pub matrix_subtraction: BenchmarkData,
    pub matrix_multiplication: BenchmarkData,
    pub scalar_multiplication: BenchmarkData,
}

impl BenchmarkResults {
    pub fn new(
        matrix_addition: BenchmarkData,
        matrix_subtraction: BenchmarkData,
        matrix_multiplication: BenchmarkData,
        scalar_multiplication: BenchmarkData,
    ) -> Self {
        Self {
            matrix_addition,
            matrix_subtraction,
            matrix_multiplication,
            scalar_multiplication,
        }
    }
}

#[derive(Serialize)]
pub struct BenchmarkData {
    pub sizes: Vec<usize>,
    pub times: Vec<f64>,
}

impl BenchmarkData {
    pub fn new(sizes: Vec<usize>, times: Vec<f64>) -> Self {
        Self { sizes, times }
    }
}
