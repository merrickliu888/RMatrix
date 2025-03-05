mod benchmarking;
mod matrices;

use benchmarking::save_benchmark_results;
use matrices::basic_matrix::benchmark_basic_matrix;
use matrices::ndarray_matrix::benchmark_ndarray_matrix;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Starting benchmark...");

    let args: Vec<String> = env::args().collect();
    let run_all = args.contains(&String::from("all"));

    println!("Loading matrices...");
    let matrices1 = load_matrices("./benches/matrices1.json");
    let matrices2 = load_matrices("./benches/matrices2.json");
    println!("Matrices loaded.");

    if !args.contains(&String::from("exclude_basic_matrix"))
        && (run_all || args.contains(&String::from("basic_matrix")))
    {
        println!("Benchmarking basic matrix...");
        let basic_matrix_results = benchmark_basic_matrix(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/basic_matrix_results.json",
            &basic_matrix_results,
        );
        println!("basic matrix benchmark completed.");
    }

    if !args.contains(&String::from("exclude_ndarray_matrix"))
        && (run_all || args.contains(&String::from("ndarray_matrix")))
    {
        println!("Benchmarking ndarray matrix...");
        let ndarray_matrix_results = benchmark_ndarray_matrix(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/ndarray_matrix_results.json",
            &ndarray_matrix_results,
        );
        println!("ndarray matrix benchmark completed.");
    }

    println!("Benchmark completed.");
}

fn load_matrices(filename: &str) -> Vec<Vec<Vec<f64>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let matrices: Vec<Vec<Vec<f64>>> = serde_json::from_reader(reader).unwrap();
    matrices
}
