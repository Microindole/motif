pub mod parse;
pub mod rule;
pub mod scan;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().expect("failed to resolve current directory"));

    match scan::scan_root(&root) {
        Ok(result) => {
            println!("scanned {} files", result.files.len());

            let mut resolved_count = 0usize;
            for class_name in result.class_names {
                match parse::parse_class_name(&class_name) {
                    Ok(parsed) => match rule::resolve_rule(&parsed) {
                        Some(rule) => {
                            resolved_count += 1;
                            println!("{} => {}", class_name, rule.describe());
                        }
                        None => println!("{} => unresolved rule", class_name),
                    },
                    Err(error) => println!("{} => parse error: {}", class_name, error),
                }
            }

            println!("resolved {} classes", resolved_count);
        }
        Err(error) => {
            eprintln!("scan failed: {error}");
            std::process::exit(1);
        }
    }
}
