mod demo_builds;
mod quality;
mod utils;

use std::env;

fn main() {
    if let Err(error) = run() {
        eprintln!("xtask failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("demo-builds") => demo_builds::run(),
        Some("quality") => quality::run(),
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown xtask command `{other}`")),
    }
}

fn print_help() {
    println!("motif xtask commands:");
    println!("  cargo run -p xtask -- quality");
    println!("  cargo run -p xtask -- demo-builds");
}
