mod benchmarking;

use benchmarking::{benchmark_matrix, load_matrices, save_benchmark_results};
use rmatrix::matrices::basic_matrix::BasicMatrix;
use rmatrix::matrices::blas_matrix::BlasMatrix;
use rmatrix::matrices::blocked_matrix::BlockedMatrix;
use rmatrix::matrices::ndarray_matrix::NdarrayMatrix;
use rmatrix::matrices::one_d_vec_matrix::OneDVecMatrix;
use rmatrix::matrices::transposed_view_matrix::TransposedViewMatrix;

use std::env;

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
        let basic_matrix_results = benchmark_matrix::<BasicMatrix>(&matrices1, &matrices2);
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
        let ndarray_matrix_results = benchmark_matrix::<NdarrayMatrix>(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/ndarray_matrix_results.json",
            &ndarray_matrix_results,
        );
        println!("ndarray matrix benchmark completed.");
    }

    if !args.contains(&String::from("exclude_one_d_vec_matrix"))
        && (run_all || args.contains(&String::from("one_d_vec_matrix")))
    {
        println!("Benchmarking one_d_vec_matrix...");
        let one_d_vec_matrix_results = benchmark_matrix::<OneDVecMatrix>(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/one_d_vec_matrix_results.json",
            &one_d_vec_matrix_results,
        );
        println!("one_d_vec_matrix benchmark completed.");
    }

    if !args.contains(&String::from("exclude_blocked_matrix"))
        && (run_all || args.contains(&String::from("blocked_matrix")))
    {
        println!("Benchmarking blocked matrix...");
        let blocked_matrix_results = benchmark_matrix::<BlockedMatrix>(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/blocked_matrix_results.json",
            &blocked_matrix_results,
        );
        println!("blocked matrix benchmark completed.");
    }

    if !args.contains(&String::from("exclude_transposed_view_matrix"))
        && (run_all || args.contains(&String::from("transposed_view_matrix")))
    {
        println!("Benchmarking transposed view matrix...");
        let transposed_view_matrix_results =
            benchmark_matrix::<TransposedViewMatrix>(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/transposed_view_matrix_results.json",
            &transposed_view_matrix_results,
        );
        println!("transposed view matrix benchmark completed.");
    }

    if !args.contains(&String::from("exclude_blas_matrix"))
        && (run_all || args.contains(&String::from("blas_matrix")))
    {
        println!("Benchmarking blas matrix...");
        let blas_matrix_results = benchmark_matrix::<BlasMatrix>(&matrices1, &matrices2);
        save_benchmark_results(
            "./benches/benchmark_results/blas_matrix_results.json",
            &blas_matrix_results,
        );
        println!("blas matrix benchmark completed.");
    }

    println!("Benchmark completed.");
}
