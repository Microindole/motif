use crate::utils::{npm_program, path_from_repo, repo_root, run_step};

const DEMOS: &[(&str, &str)] = &[
    ("ts-basic", "demo/ts/basic"),
    ("ts-variants", "demo/ts/variants"),
    ("ts-theme", "demo/ts/theme"),
    ("ts-workspace", "demo/ts/workspace"),
    ("react-basic", "demo/react/basic"),
    ("react-variants", "demo/react/variants"),
    ("react-theme", "demo/react/theme"),
    ("react-workspace", "demo/react/workspace"),
    ("vue-basic", "demo/vue/basic"),
    ("vue-variants", "demo/vue/variants"),
    ("vue-theme", "demo/vue/theme"),
    ("vue-workspace", "demo/vue/workspace"),
];

pub fn run() -> Result<(), String> {
    let root = repo_root();
    let npm = npm_program();

    for (name, relative_path) in DEMOS {
        let demo_dir = path_from_repo(&root, relative_path);
        run_step(
            &format!("{name}: generate motif.css"),
            "cargo",
            &["run", "-p", "motif-core", "--", "."],
            &demo_dir,
        )?;
        run_step(
            &format!("{name}: install dependencies"),
            npm,
            &["install", "--no-package-lock"],
            &demo_dir,
        )?;
        run_step(&format!("{name}: build"), npm, &["run", "build"], &demo_dir)?;
    }

    Ok(())
}
