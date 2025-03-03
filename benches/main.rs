mod benchmarking;
mod matrices;

use benchmarking::save_benchmark_results;
use matrices::basic_matrix::benchmark_basic_matrix;
use matrices::ndarray_matrix::benchmark_ndarray_matrix;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let matrices1 = load_matrices("./benches/matrices1.json");
    let matrices2 = load_matrices("./benches/matrices2.json");

    let basic_matrix_results = benchmark_basic_matrix(&matrices1, &matrices2);
    let ndarray_matrix_results = benchmark_ndarray_matrix(&matrices1, &matrices2);

    save_benchmark_results(
        "./benches/benchmark_results/basic_matrix_results.json",
        &basic_matrix_results,
    );
    save_benchmark_results(
        "./benches/benchmark_results/ndarray_matrix_results.json",
        &ndarray_matrix_results,
    );
}

fn load_matrices(filename: &str) -> Vec<Vec<Vec<f64>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let matrices: Vec<Vec<Vec<f64>>> = serde_json::from_reader(reader).unwrap();
    matrices
}
