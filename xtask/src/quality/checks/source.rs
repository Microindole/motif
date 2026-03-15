pub(super) fn is_source_file(file: &str) -> bool {
    if file.starts_with("node_modules/")
        || file.contains("/node_modules/")
        || file.starts_with("target/")
        || file.contains("/target/")
        || file.starts_with("dist/")
        || file.contains("/dist/")
        || file.starts_with("coverage/")
        || file.contains("/coverage/")
        || file.starts_with(".vite/")
        || file.contains("/.vite/")
    {
        return false;
    }

    is_code_extension(file)
}

pub(super) fn is_code_extension(file: &str) -> bool {
    [
        ".rs", ".ts", ".tsx", ".js", ".jsx", ".vue", ".svelte", ".ps1", ".sh",
    ]
    .iter()
    .any(|ext| file.ends_with(ext))
}

pub(super) fn source_file_line_limit(file: &str) -> Option<usize> {
    match file {
        value
            if value.starts_with("core/src/rule/")
                && value.ends_with(".rs")
                && value.matches('/').count() >= 4 =>
        {
            Some(360)
        }
        value if value.starts_with("core/src/rule/") && value.ends_with(".rs") => Some(240),
        value if value.starts_with("core/src/") && value.ends_with(".rs") => Some(320),
        value if value.starts_with("core/tests/") && value.ends_with(".rs") => Some(420),
        value if value.starts_with("xtask/src/") && value.ends_with(".rs") => Some(320),
        value if value.starts_with("xtask/tests/") && value.ends_with(".rs") => Some(320),
        value
            if value.starts_with("scripts/")
                && (value.ends_with(".ps1") || value.ends_with(".sh")) =>
        {
            Some(320)
        }
        value
            if value.starts_with("packages/motif-vite/src/core/")
                && is_code_extension(value)
                && value.matches('/').count() >= 6 =>
        {
            Some(280)
        }
        value
            if value.starts_with("packages/motif-vite/src/")
                && is_code_extension(value)
                && value.matches('/').count() >= 5 =>
        {
            Some(220)
        }
        value if value.starts_with("packages/") && is_code_extension(value) => Some(360),
        value
            if (value.starts_with("demo/") || value.starts_with("cases/"))
                && is_source_file(value) =>
        {
            Some(260)
        }
        value if is_source_file(value) => Some(320),
        _ => None,
    }
}
