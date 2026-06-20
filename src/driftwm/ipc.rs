//! Driftwm IPC protocol — socket connect + request.

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
    pub windows: Vec<WindowInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindowInfo {
    pub app_id: String,
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

/// Send a `State` request and parse the response.
pub fn request_state() -> Result<StateResponse, String> {
    let reply = send_request(&Request::State)?;
    serde_json::from_str(&reply).map_err(|e| format!("parse state response: {e} (raw: {reply:?})"))
}

/// Execute a compositor action (e.g. `"reload-config"`, `"close-window"`).
pub fn run_action(action: &str) -> Result<(), String> {
    send_request(&Request::Action(action.to_string()))?;
    Ok(())
}
