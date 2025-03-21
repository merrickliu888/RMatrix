fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-search=framework=/System/Library/Frameworks");
    } else {
        panic!("Apple Accelerate framework is only available on macOS");
    }
}
