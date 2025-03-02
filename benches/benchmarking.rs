use serde::Serialize;
use std::fs::File;

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
