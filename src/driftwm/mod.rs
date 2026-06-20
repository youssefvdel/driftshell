//! Driftwm integration: IPC client + TOML config.

pub mod config;
pub mod ipc;

/// Shared state tracked from driftwm.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub workspaces: Vec<Workspace>,
    #[allow(dead_code)]
    pub focused_app_id: Option<String>,
    pub windows: Vec<ipc::WindowInfo>,
}

/// A visible client window on the driftwm canvas.
#[derive(Debug, Clone, Default)]
pub struct Workspace {
    #[allow(dead_code)]
    pub id: usize,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    StateUpdate(State),
    Error(String),
}

pub fn update(state: &mut State, msg: Message) {
    match msg {
        Message::StateUpdate(new_state) => *state = new_state,
        Message::Error(e) => log::error!("driftwm: {e}"),
    }
}
