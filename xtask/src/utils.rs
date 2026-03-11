use std::path::{Path, PathBuf};
use std::process::Command;

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask should live under repo root")
        .to_path_buf()
}

pub fn command_output(program: &str, args: &[&str], cwd: &Path) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|error| format!("failed to run `{program}`: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "`{program} {}` failed:\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_step(name: &str, program: &str, args: &[&str], cwd: &Path) -> Result<(), String> {
    println!("==> {name}");
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .map_err(|error| format!("failed to run `{program}`: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{name} failed with exit code {:?}", status.code()))
    }
}

pub fn tracked_files(root: &Path) -> Result<Vec<String>, String> {
    let output = command_output("git", &["ls-files"], root)?;
    Ok(output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.replace('\\', "/"))
        .collect())
}

pub fn read_lines(path: &Path) -> Result<Vec<String>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

pub fn path_from_repo(root: &Path, relative: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    for part in relative.split('/') {
        path.push(part);
    }
    path
}

pub fn npm_program() -> &'static str {
    if cfg!(windows) {
        "npm.cmd"
    } else {
        "npm"
    }
}
