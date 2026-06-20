//! Root Iced Daemon — manages bar + launcher windows.

use iced::Element;
use iced::Task as Command;
use iced_layershell::reexport::*;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::settings::StartMode;
use iced_layershell::{daemon, to_layer_message};

use crate::bar;
use crate::dock;
use crate::driftwm;
use crate::launcher;
use crate::settings;
use crate::shared;
use crate::tray;

// ── Messages ───────────────────────────────────────────────────────────────

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    Bar(bar::Message),
    Dock(dock::Message),
    Driftwm(driftwm::Message),
    Launcher(launcher::Message),
    Settings(settings::Message),
    Tick,
    OpenLauncher,
    OpenSettings,
}

// ── App State ──────────────────────────────────────────────────────────────

pub struct App {
    pub bar: bar::Bar,
    pub dock: dock::Dock,
    pub driftwm: driftwm::State,
    pub launcher: launcher::Launcher,
    pub settings: settings::Settings,
    pub apps_scanned: bool,
    pub tray_state: tray::TrayState,
    pub dock_window: Option<IcedId>,
    pub launcher_window: Option<IcedId>,
    pub settings_window: Option<IcedId>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            bar: bar::Bar::default(),
            dock: dock::Dock::default(),
            driftwm: driftwm::State::default(),
            launcher: launcher::Launcher::new(std::collections::HashMap::new()),
            settings: settings::Settings::default(),
            apps_scanned: false,
            tray_state: tray::new_state(),
            dock_window: None,
            launcher_window: None,
            settings_window: None,
        }
    }
}

// ── Public Entrypoint ──────────────────────────────────────────────────────

pub fn run() -> Result<(), iced_layershell::Error> {
    daemon(
        || {
            let (dock_id, dock_cmd) = Message::layershell_open(NewLayerShellSettings {
                size: Some((0, 48)),
                exclusive_zone: Some(48),
                anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
                layer: Layer::Top,
                keyboard_interactivity: KeyboardInteractivity::None,
                events_transparent: false,
                output_option: OutputOption::None,
                namespace: Some("driftshell-dock".to_string()),
                margin: None,
            });
            let mut app = App::default();
            tray::spawn_watcher(app.tray_state.clone());
            let favs = dock::scan_favorites();
            if !favs.is_empty() {
                app.dock.favorites = favs;
            }
            app.dock_window = Some(dock_id);
            (
                app,
                Command::batch([schedule_tick(), poll_driftwm(), dock_cmd]),
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
                match msg {
                    bar::Message::ToggleLauncher => {
                        bar::update(&mut self.bar, msg);
                        return self.toggle_launcher();
                    }
                    bar::Message::ToggleSettings => {
                        bar::update(&mut self.bar, msg);
                        return self.toggle_settings();
                    }
                    _ => bar::update(&mut self.bar, msg),
                }
                Command::none()
            }
            Message::Dock(msg) => {
                dock::update(&mut self.dock, msg);
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
            Message::Settings(msg) => self.handle_settings(msg),
            Message::Tick => {
                bar::update(&mut self.bar, bar::Message::Tick);
                let services: Vec<String> = tray::read_items(&self.tray_state)
                    .into_iter()
                    .map(|t| t.service.clone())
                    .collect();
                bar::update(&mut self.bar, bar::Message::TrayUpdate(services));
                Command::batch([poll_driftwm(), schedule_tick()])
            }
            Message::OpenLauncher => self.toggle_launcher(),
            Message::OpenSettings => self.toggle_settings(),
            _ => Command::none(),
        }
    }

    fn view(&self, id: IcedId) -> Element<'_, Message> {
        if self.settings.visible && Some(id) == self.settings_window {
            settings::view(&self.settings).map(Message::Settings)
        } else if self.launcher.visible && Some(id) == self.launcher_window {
            launcher::view(&self.launcher).map(Message::Launcher)
        } else if Some(id) == self.dock_window {
            dock::view(&self.dock).map(Message::Dock)
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

    fn handle_settings(&mut self, msg: settings::Message) -> Command<Message> {
        match &msg {
            settings::Message::Close | settings::Message::Save => {
                settings::update(&mut self.settings, &msg);
                let id = self.settings_window.take();
                let mut cmds = Vec::new();
                if let Some(id) = id {
                    cmds.push(Command::done(Message::RemoveWindow(id)));
                }
                if matches!(&msg, settings::Message::Save) {
                    cmds.push(poll_driftwm());
                }
                Command::batch(cmds)
            }
            _ => {
                settings::update(&mut self.settings, &msg);
                Command::none()
            }
        }
    }

    fn toggle_settings(&mut self) -> Command<Message> {
        if !self.settings.visible {
            let config = driftwm::config::read().ok();
            settings::update(&mut self.settings, &settings::Message::Open(config));

            let (id, task) = Message::layershell_open(NewLayerShellSettings {
                size: Some((600, 500)),
                exclusive_zone: Some(0),
                anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
                layer: Layer::Overlay,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                margin: None,
                events_transparent: false,
                output_option: OutputOption::None,
                namespace: Some("driftshell-settings".to_string()),
            });
            self.settings_window = Some(id);
            task
        } else {
            settings::update(&mut self.settings, &settings::Message::Close);
            self.settings_window
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
            dock::update(&mut self.dock, dock::Message::State(state.windows.clone()));
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
                        windows: response.windows,
                    })
                }
                Ok(Err(e)) => driftwm::Message::Error(e),
                Err(_) => driftwm::Message::Error("blocking task cancelled".to_string()),
            }
        },
        Message::Driftwm,
    )
}
