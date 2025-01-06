use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INVALID_CHARS: Regex = Regex::new(r#"[<>:"/\\|?*]"#).unwrap();
}

pub fn sanitize_filename(name: &str) -> String {
    let sanitized = INVALID_CHARS.replace_all(name, "_");
    let trimmed = sanitized.trim_matches(|c| c == '.' || c == ' ');
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_path_display(path: &Path) -> String {
    path.components()
        .map(|comp| comp.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
