//! Driftwm TOML config model + parser.
//!
//! #![allow(dead_code)] — planned features: settings UI + wallpaper changer
//! will call config_path(), read(), and DriftwmConfig once built.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DriftwmConfig {
    pub cursor: Option<CursorConfig>,
    pub background: Option<BackgroundConfig>,
    pub decorations: Option<DecorationsConfig>,
    pub bar: Option<BarConfig>,
    pub general: Option<GeneralConfig>,
    pub autostart: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecorationsConfig {
    pub title_bar: Option<TitleBarConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TitleBarConfig {
    pub font: Option<String>,
    pub font_size: Option<i32>,
    pub font_weight: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BarConfig {
    pub opacity: Option<f64>,
    pub capsule_show: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    pub animation_speed: Option<f64>,
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

/// Serialize and write config to disk.
pub fn write(config: &DriftwmConfig) -> Result<(), String> {
    let path = config_path();
    let content = toml::to_string_pretty(config).map_err(|e| format!("serialize config: {}", e))?;
    std::fs::write(&path, content).map_err(|e| format!("write config {}: {}", path.display(), e))
}
