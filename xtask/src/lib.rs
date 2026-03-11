pub mod demo_builds;
pub mod quality;
pub mod utils;

pub fn run_cli(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let mut args = args.into_iter();
    let _program = args.next();
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

pub fn print_help() {
    println!("motif xtask commands:");
    println!("  cargo run -p xtask -- quality");
    println!("  cargo run -p xtask -- demo-builds");
}
