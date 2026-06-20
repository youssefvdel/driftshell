//! Root Iced Daemon — manages bar + launcher windows.

use iced::Element;
use iced::Task as Command;
use iced_layershell::reexport::*;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::settings::StartMode;
use iced_layershell::{daemon, to_layer_message};

use crate::bar;
use crate::driftwm;
use crate::launcher;
use crate::shared;

// ── Messages ───────────────────────────────────────────────────────────────

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    Bar(bar::Message),
    Driftwm(driftwm::Message),
    Launcher(launcher::Message),
    Tick,
    OpenLauncher,
}

// ── App State ──────────────────────────────────────────────────────────────

pub struct App {
    pub bar: bar::Bar,
    pub driftwm: driftwm::State,
    pub launcher: launcher::Launcher,
    pub apps_scanned: bool,
    pub launcher_window: Option<IcedId>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            bar: bar::Bar::default(),
            driftwm: driftwm::State::default(),
            launcher: launcher::Launcher::new(std::collections::HashMap::new()),
            apps_scanned: false,
            launcher_window: None,
        }
    }
}

// ── Public Entrypoint ──────────────────────────────────────────────────────

pub fn run() -> Result<(), iced_layershell::Error> {
    daemon(
        || {
            (
                App::default(),
                Command::batch([schedule_tick(), poll_driftwm()]),
            )
        },
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

// ── Daemon Logic ───────────────────────────────────────────────────────────

impl App {
    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Bar(msg) => {
                if matches!(msg, bar::Message::ToggleLauncher) {
                    bar::update(&mut self.bar, msg);
                    return self.toggle_launcher();
                }
                bar::update(&mut self.bar, msg);
                Command::none()
            }
            Message::Driftwm(msg) => {
                self.handle_driftwm(msg);
                Command::none()
            }
            Message::Launcher(msg) => {
                let was_visible = self.launcher.visible;
                launcher::update(&mut self.launcher, &msg);

                match &msg {
                    launcher::Message::Toggle | launcher::Message::Close => {
                        if was_visible {
                            let id = self.launcher_window.take();
                            if let Some(id) = id {
                                return Command::done(Message::RemoveWindow(id));
                            }
                        }
                    }
                    launcher::Message::Launch(_) => {
                        let id = self.launcher_window.take();
                        if let Some(id) = id {
                            return Command::done(Message::RemoveWindow(id));
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::Tick => {
                bar::update(&mut self.bar, bar::Message::Tick);
                Command::batch([poll_driftwm(), schedule_tick()])
            }
            Message::OpenLauncher => self.toggle_launcher(),
            _ => Command::none(),
        }
    }

    fn view(&self, id: IcedId) -> Element<'_, Message> {
        if self.launcher.visible && Some(id) == self.launcher_window {
            launcher::view(&self.launcher).map(Message::Launcher)
        } else {
            bar::view(&self.bar).map(Message::Bar)
        }
    }

    fn toggle_launcher(&mut self) -> Command<Message> {
        if !self.launcher.visible {
            if !self.apps_scanned {
                let apps = shared::apps::scan_apps();
                self.launcher = launcher::Launcher::new(apps);
                self.apps_scanned = true;
            }
            launcher::update(&mut self.launcher, &launcher::Message::Toggle);

            let (id, task) = Message::layershell_open(NewLayerShellSettings {
                size: Some((500, 600)),
                exclusive_zone: Some(0),
                anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
                layer: Layer::Overlay,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                margin: Some((0, 0, 0, 0)),
                events_transparent: false,
                output_option: OutputOption::None,
                namespace: Some("driftshell-launcher".to_string()),
            });
            self.launcher_window = Some(id);
            task
        } else {
            launcher::update(&mut self.launcher, &launcher::Message::Toggle);
            self.launcher_window
                .take()
                .map(|id| Command::done(Message::RemoveWindow(id)))
                .unwrap_or_default()
        }
    }

    fn handle_driftwm(&mut self, msg: driftwm::Message) {
        if let driftwm::Message::StateUpdate(state) = &msg {
            bar::update(
                &mut self.bar,
                bar::Message::Workspaces(state.workspaces.clone()),
            );
        }
        driftwm::update(&mut self.driftwm, msg);
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
