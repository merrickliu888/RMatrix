fn main() {
    if !cfg!(target_os = "macos") {
        panic!("Apple Accelerate framework is only available on macOS");
    }
}
