//! Driftwm TOML config model + parser.
//!
//! #![allow(dead_code)] — needed when settings + wallpaper modules are built
//! (config_path() + read() + BackgroundConfig).
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DriftwmConfig {
    pub cursor: Option<CursorConfig>,
    pub background: Option<BackgroundConfig>,
    pub autostart: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorConfig {
    pub theme: Option<String>,
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackgroundConfig {
    #[serde(rename = "type")]
    pub bg_type: Option<String>,
    pub path: Option<String>,
}

pub fn config_path() -> PathBuf {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap_or_default()));
    PathBuf::from(config_home).join("driftwm/config.toml")
}

pub fn read() -> Result<DriftwmConfig, String> {
    let path = config_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("read config {}: {}", path.display(), e))?;
    toml::from_str(&content).map_err(|e| format!("parse config: {}", e))
}
