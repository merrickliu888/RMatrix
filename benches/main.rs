use std::fs::File;
use std::io::BufReader;

fn main() {
    let matrices = load_matrices("./benches/matrices.json");
    println!("Num matrices: {}", matrices.len());
}

fn load_matrices(filename: &str) -> Vec<Vec<Vec<f64>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let matrices: Vec<Vec<Vec<f64>>> = serde_json::from_reader(reader).unwrap();
    matrices
}
