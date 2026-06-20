//! Root Iced Application — runs the bar via iced_layershell.

use iced::Element;
use iced::Task as Command;
use iced_layershell::reexport::*;
use iced_layershell::settings::{LayerShellSettings, StartMode};
use iced_layershell::{application, to_layer_message};

use crate::bar;
use crate::driftwm;

// ── Messages ───────────────────────────────────────────────────────────────

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Bar(bar::Message),
    Driftwm(driftwm::Message),
    Tick,
}

// ── App State ──────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct App {
    pub bar: bar::Bar,
    pub driftwm: driftwm::State,
}

// ── Public Entrypoint ──────────────────────────────────────────────────────

pub fn run() -> Result<(), iced_layershell::Error> {
    application(
        || (App::default(), schedule_tick()),
        "driftshell",
        App::update,
        App::view,
    )
    .layer_settings(LayerShellSettings {
        size: Some((0, 36)),
        exclusive_zone: 36,
        anchor: Anchor::Top | Anchor::Left | Anchor::Right,
        layer: Layer::Top,
        keyboard_interactivity: KeyboardInteractivity::None,
        start_mode: StartMode::Active,
        ..Default::default()
    })
    .run()
}

// ── Application Logic ──────────────────────────────────────────────────────

impl App {
    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Bar(msg) => {
                bar::update(&mut self.bar, msg);
                Command::none()
            }
            Message::Driftwm(msg) => {
                if let driftwm::Message::StateUpdate(state) = &msg {
                    bar::update(
                        &mut self.bar,
                        bar::Message::Workspaces(state.workspaces.clone()),
                    );
                }
                driftwm::update(&mut self.driftwm, msg);
                Command::none()
            }
            Message::Tick => {
                bar::update(&mut self.bar, bar::Message::Tick);
                Command::batch([poll_driftwm(), schedule_tick()])
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        bar::view(&self.bar).map(Message::Bar)
    }
}

// ── Scheduling ─────────────────────────────────────────────────────────────

fn schedule_tick() -> Command<Message> {
    Command::perform(
        async {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        },
        |_| Message::Tick,
    )
}

/// Poll driftwm state in a background thread and emit a `Driftwm(StateUpdate)` message.
fn poll_driftwm() -> Command<Message> {
    Command::perform(
        async {
            let result = tokio::task::spawn_blocking(driftwm::ipc::request_state).await;
            match result {
                Ok(Ok(response)) => {
                    let workspaces: Vec<_> = response
                        .windows
                        .iter()
                        .filter(|w| !w.is_widget)
                        .enumerate()
                        .map(|(i, w)| driftwm::Workspace {
                            id: i,
                            name: w.app_id.clone(),
                            active: w.is_focused,
                        })
                        .collect();
                    let focused = response
                        .windows
                        .iter()
                        .find(|w| w.is_focused)
                        .map(|w| w.app_id.clone());
                    driftwm::Message::StateUpdate(driftwm::State {
                        workspaces,
                        focused_app_id: focused,
                    })
                }
                Ok(Err(e)) => driftwm::Message::Error(e),
                Err(_) => driftwm::Message::Error("blocking task cancelled".to_string()),
            }
        },
        Message::Driftwm,
    )
}
