//! System tray — zbus StatusNotifierWatcher integration.
//!
//! Connects to the D-Bus session bus, registers as a StatusNotifierHost,
//! and watches for tray items. Uses a blocking connection on a background thread.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A single tray item as seen by the host.
#[derive(Debug, Clone, Default)]
pub struct TrayItem {
    pub service: String,
    #[allow(dead_code)] // used later for tooltip rendering
    pub title: String,
}

/// Shared state: service name → TrayItem.
pub type TrayState = Arc<Mutex<HashMap<String, TrayItem>>>;

/// Create a new empty TrayState.
pub fn new_state() -> TrayState {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Spawn a background thread that runs the zbus StatusNotifierHost.
/// On failure it retries every 5 seconds.
pub fn spawn_watcher(state: TrayState) {
    std::thread::spawn(move || {
        loop {
            if let Err(e) = run_watcher_blocking(&state) {
                log::warn!("tray watcher: {e}; retrying in 5s");
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });
}

fn run_watcher_blocking(state: &TrayState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = zbus::blocking::Connection::session()?;

    // Register as a StatusNotifierHost (best-effort, watcher may not exist)
    let _ = conn.call_method(
        Some("org.freedesktop.StatusNotifierWatcher"),
        "/StatusNotifierWatcher",
        Some("org.freedesktop.StatusNotifierWatcher"),
        "RegisterStatusNotifierHost",
        &("driftshell",),
    );
    log::info!("tray: registered as StatusNotifierHost");

    // Poll D-Bus for StatusNotifierItem services every 2s
    loop {
        if let Ok(reply) = conn.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        ) {
            if let Ok(names) = reply.body().deserialize::<Vec<String>>() {
                let mut new_items = HashMap::new();
                for name in &names {
                    if name.starts_with("org.freedesktop.StatusNotifierItem-") {
                        let item = TrayItem {
                            service: name.clone(),
                            title: name
                                .strip_prefix("org.freedesktop.StatusNotifierItem-")
                                .unwrap_or(name)
                                .to_string(),
                        };
                        new_items.insert(name.clone(), item);
                    }
                }
                if let Ok(mut items) = state.lock() {
                    *items = new_items;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

/// Read the current items (best-effort, non-blocking).
pub fn read_items(state: &TrayState) -> Vec<TrayItem> {
    match state.lock() {
        Ok(items) => {
            let mut list: Vec<TrayItem> = items.values().cloned().collect();
            list.sort_by(|a, b| a.service.cmp(&b.service));
            list
        }
        Err(_) => vec![],
    }
}
