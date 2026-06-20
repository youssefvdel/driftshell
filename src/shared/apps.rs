//! Desktop file discovery and parsing (XDG Desktop Entry spec).
//!
//! Scans `$XDG_DATA_HOME/applications/` + `$XDG_DATA_DIRS/applications/` for `.desktop` files.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    #[allow(dead_code)] // used for icon rendering
    pub icon: Option<String>,
    pub categories: Vec<String>,
    pub filename: String,
}

/// Scan all XDG application directories for `.desktop` files.
/// Returns a map of `app_id → AppEntry` (lowercased filename without .desktop).
pub fn scan_apps() -> HashMap<String, AppEntry> {
    let mut apps = HashMap::new();

    let data_dirs = xdg_data_dirs();
    for dir in &data_dirs {
        let apps_dir = dir.join("applications");
        if !apps_dir.is_dir() {
            continue;
        }
        let entries = match fs::read_dir(&apps_dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }
            let app_id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase());
            let Some(app_id) = app_id else { continue };
            if apps.contains_key(&app_id) {
                continue; // earlier dirs take priority
            }
            if let Some(entry) = parse_desktop_file(&path) {
                apps.insert(app_id, entry);
            }
        }
    }

    apps
}

/// Parse a single `.desktop` file.
fn parse_desktop_file(path: &Path) -> Option<AppEntry> {
    let content = fs::read_to_string(path).ok()?;
    let mut in_desktop_entry = false;
    let mut entry_type = String::new();
    let mut name = String::new();
    let mut exec = String::new();
    let mut icon = None;
    let mut no_display = false;
    let mut categories = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        // Section header
        if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_desktop_entry {
            continue;
        }
        // Comment or blank
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Key=Value (ignoring localized keys like Name[fr] for now)
        let eq = line.find('=')?;
        let raw_key = &line[..eq];
        let value = &line[eq + 1..];

        // Skip localized variants (key[locale])
        let key = if let Some(bracket) = raw_key.find('[') {
            &raw_key[..bracket]
        } else {
            raw_key
        };

        match key {
            "Type" => entry_type = value.to_string(),
            "Name" if name.is_empty() => name = value.to_string(),
            "Exec" => exec = sanitize_exec(value),
            "Icon" => icon = Some(value.to_string()),
            "NoDisplay" => no_display = value.trim() == "true",
            "Categories" => {
                categories = value
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            _ => {}
        }
    }

    if entry_type != "Application" || name.is_empty() || exec.is_empty() || no_display {
        return None;
    }

    Some(AppEntry {
        name,
        exec,
        icon,
        categories,
        filename: path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string(),
    })
}

/// Strip `%` field codes from Exec (e.g. `%u`, `%f`, `%U`, `%F`) and trim.
fn sanitize_exec(exec: &str) -> String {
    let mut result = String::with_capacity(exec.len());
    let mut chars = exec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            // Skip the `%` and the following field code letter
            let _ = chars.next();
        } else {
            result.push(c);
        }
    }
    result.trim().to_string()
}

/// Collect all XDG data directories for application entries.
fn xdg_data_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // $XDG_DATA_HOME (default ~/.local/share)
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share"));
    }
    if let Ok(xdg_home) = std::env::var("XDG_DATA_HOME") {
        dirs.push(PathBuf::from(xdg_home));
    }

    // $XDG_DATA_DIRS (default /usr/local/share:/usr/share)
    let xdg_dirs = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());
    for d in xdg_dirs.split(':') {
        let p = PathBuf::from(d);
        if !dirs.contains(&p) {
            dirs.push(p);
        }
    }

    dirs
}
