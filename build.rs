fn main() {
    println!(
        "cargo:rustc-env=PLATFORM={}",
        std::env::var("TARGET").unwrap()
    );
}
