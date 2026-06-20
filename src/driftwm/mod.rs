//! Driftwm integration: IPC client + TOML config.
//!
//! #![allow(dead_code)] — State + Workspace + Message + update() all built for the
//! next feature: bar workspace polling + indicator display.
#![allow(dead_code)]

pub mod config;
pub mod ipc;

use std::time::Duration;

/// Polling interval for driftwm state file updates — used when workspace polling is wired up.
pub const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Shared state tracked from driftwm. Used by the bar's workspace indicator module.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub workspaces: Vec<Workspace>,
    pub focused_app_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Poll,
    StateUpdate(State),
    Error(String),
}

pub fn update(state: &mut State, msg: Message) {
    match msg {
        Message::Poll => {}
        Message::StateUpdate(new_state) => *state = new_state,
        Message::Error(e) => log::error!("driftwm: {e}"),
    }
}
