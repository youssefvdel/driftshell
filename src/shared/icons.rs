/// XDG icon theme lookup for iced::widget::image.
use std::path::PathBuf;

/// Directories searched (XDG_DATA_DIRS + known paths).
const ICON_DIRS: &[&str] = &[
    "/usr/share/icons/hicolor/48x48/apps",
    "/usr/share/icons/hicolor/64x64/apps",
    "/usr/share/icons/hicolor/scalable/apps",
    "/usr/share/pixmaps",
    "/usr/share/icons/Adwaita/48x48/apps",
    "/usr/share/icons/breeze/48x48/apps",
];

/// User-local overrides.
fn user_icon_dirs() -> Vec<PathBuf> {
    let home = std::env::var("HOME").unwrap_or_default();
    vec![
        PathBuf::from(&home).join(".local/share/icons/hicolor/48x48/apps"),
        PathBuf::from(&home).join(".local/share/icons/hicolor/64x64/apps"),
    ]
}

pub fn find(app_id: &str) -> Option<PathBuf> {
    let candidates = icon_candidates(app_id);

    let user_dirs = user_icon_dirs();
    let system_dirs: Vec<PathBuf> = ICON_DIRS.iter().map(|s| PathBuf::from(s)).collect();
    for dir in user_dirs.iter().chain(system_dirs.iter()) {
        let dir = PathBuf::from(dir);
        if !dir.is_dir() {
            continue;
        }
        for name in &candidates {
            let png = dir.join(format!("{name}.png"));
            if png.is_file() {
                return Some(png);
            }
            // Also try .svg (for scalable)
            let svg = dir.join(format!("{name}.svg"));
            if svg.is_file() {
                return Some(svg);
            }
        }
    }
    None
}

/// Generate possible icon names for an app_id.
fn icon_candidates(app_id: &str) -> Vec<String> {
    let mut names = Vec::with_capacity(4);
    names.push(app_id.to_string());
    names.push(app_id.to_lowercase());

    // Reverse domain -> simple name (e.g. "org.mozilla.firefox" -> "firefox")
    if let Some(last) = app_id.rsplit('.').next() {
        if !names.contains(&last.to_string()) {
            names.push(last.to_string());
        }
        if !names.contains(&last.to_lowercase()) {
            names.push(last.to_lowercase());
        }
    }
    names
}
