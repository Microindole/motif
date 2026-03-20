use std::path::Path;
use std::process::Command;

pub fn run_cli(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let mut args = args.into_iter();
    let _program = args.next();
    match args.next().as_deref() {
        Some("demo-builds") => run_node_compat("scripts/node/demo-builds.mjs"),
        Some("quality") => run_node_compat("scripts/node/quality.mjs"),
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown xtask command `{other}`")),
    }
}

pub fn print_help() {
    println!("motif xtask compatibility commands:");
    println!("  cargo run -p xtask -- quality");
    println!("  cargo run -p xtask -- demo-builds");
    println!("preferred local entrypoints:");
    println!("  node scripts/node/quality.mjs");
    println!("  node scripts/node/demo-builds.mjs");
}

fn run_node_compat(script: &str) -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask should live under repo root");
    let status = Command::new("node")
        .arg(script)
        .current_dir(root)
        .status()
        .map_err(|error| format!("failed to run `node {script}`: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "node compatibility command `{script}` failed with exit code {:?}",
            status.code()
        ))
    }
}
