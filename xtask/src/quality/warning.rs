pub fn info(message: impl Into<String>) -> String {
    format!("[info] {}", message.into())
}

pub fn warn(message: impl Into<String>) -> String {
    format!("[warn] {}", message.into())
}

pub fn candidate(message: impl Into<String>) -> String {
    format!("[candidate] {}", message.into())
}
