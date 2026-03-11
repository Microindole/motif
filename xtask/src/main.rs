fn main() {
    if let Err(error) = xtask::run_cli(std::env::args()) {
        eprintln!("xtask failed: {error}");
        std::process::exit(1);
    }
}
