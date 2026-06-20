//! Driftwm IPC protocol — socket connect + request.
//!
//! #![allow(dead_code)] — next feature: bar workspace polling via State socket.
//! Once `bar.update()` polls get_state() → populates workspaces → displayed as indicators.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Request {
    Action(String),
    State,
}

#[derive(Deserialize)]
pub struct StateResponse {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub layout: String,
    pub layout_short: String,
    pub windows: Vec<WindowInfo>,
    pub layers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindowInfo {
    pub app_id: String,
    pub title: String,
    pub position: [i32; 2],
    pub size: [i32; 2],
    pub is_focused: bool,
    pub is_widget: bool,
}

fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let display = std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());
    PathBuf::from(runtime_dir)
        .join("driftwm")
        .join(format!("ipc-{}.sock", display))
}

pub fn connect() -> Result<UnixStream, String> {
    let path = socket_path();
    UnixStream::connect(&path)
        .map_err(|e| format!("driftwm IPC connect to {}: {}", path.display(), e))
}

pub fn send_request(request: &Request) -> Result<String, String> {
    let mut stream = connect()?;
    let json = serde_json::to_string(request).map_err(|e| format!("serialize: {}", e))?;
    writeln!(stream, "{json}").map_err(|e| format!("write: {}", e))?;
    let mut reply = String::new();
    stream
        .read_to_string(&mut reply)
        .map_err(|e| format!("read: {}", e))?;
    Ok(reply.trim().to_string())
}
