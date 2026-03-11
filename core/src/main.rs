use std::env;
use std::path::PathBuf;

use motif_core::{gen, parse, rule, scan, token, write};

fn main() {
    let mut args = env::args_os().skip(1);
    let root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().expect("failed to resolve current directory"));
    let output_path = args.next().map(PathBuf::from);

    match run(&root, output_path.as_deref()) {
        Ok(summary) => {
            println!("scanned {} files", summary.scanned_files);
            println!("resolved {} classes", summary.resolved_classes);
            println!("wrote {}", summary.output_path.display());
        }
        Err(error) => {
            eprintln!("motif failed: {error}");
            std::process::exit(1);
        }
    }
}

struct RunSummary {
    scanned_files: usize,
    resolved_classes: usize,
    output_path: PathBuf,
}

fn run(
    root: &std::path::Path,
    output_path: Option<&std::path::Path>,
) -> Result<RunSummary, String> {
    let scan_result = scan::scan_root(root).map_err(|error| error.to_string())?;
    let token_registry = token::load_registry().map_err(|error| error.to_string())?;

    let resolved_rules = scan_result
        .class_names
        .iter()
        .filter_map(|class_name| parse::parse_class_name(class_name).ok())
        .filter_map(|parsed| rule::resolve_rule(&parsed, &token_registry))
        .collect::<Vec<_>>();

    let stylesheet = gen::render_stylesheet(&resolved_rules);
    let output_path = write::resolve_output_path(root, output_path);
    write::write_stylesheet(&output_path, &stylesheet).map_err(|error| error.to_string())?;

    Ok(RunSummary {
        scanned_files: scan_result.files.len(),
        resolved_classes: resolved_rules.len(),
        output_path,
    })
}
