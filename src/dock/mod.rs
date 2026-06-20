//! Dock — wlr-layer-shell bottom bar with favorite and running apps.
//!
//! Shows up to 10 favorite applications (from `.desktop` files) followed by
//! any running windows not already in the favorites list. Running apps are
//! highlighted with the accent color.

use iced::widget::button::{self, Button, Status};
use iced::widget::{Space, container, row, text};
use iced::{Color, Element, Length, Theme};

use crate::driftwm;
use crate::shared;

// ── Constants ──────────────────────────────────────────────────────────────

const DOCK_HEIGHT: f32 = 48.0;
const DOCK_PADDING: f32 = 12.0;

// ── State ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Dock {
    pub favorites: Vec<shared::apps::AppEntry>,
    pub running: Vec<String>,
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    State(Vec<driftwm::ipc::WindowInfo>),
    Launch(usize),
    Focus(usize),
}

// ── Update ─────────────────────────────────────────────────────────────────

pub fn update(dock: &mut Dock, msg: Message) {
    match msg {
        Message::State(windows) => {
            dock.running = windows
                .iter()
                .filter(|w| !w.is_widget)
                .map(|w| w.app_id.clone())
                .collect();
        }
        Message::Launch(idx) => {
            if let Some(app) = dock.favorites.get(idx)
                && let Err(e) = std::process::Command::new("sh")
                    .args(["-c", &format!("exec {}", app.exec)])
                    .spawn()
            {
                log::error!("launch {}: {e}", app.name);
            }
        }
        Message::Focus(idx) => {
            if let Some(app_id) = dock.running.get(idx)
                && let Err(e) = driftwm::ipc::run_action(&format!("focus-window {app_id}"))
            {
                log::error!("focus window: {e}");
            }
        }
    }
}

// ── View helpers ────────────────────────────────────────────────────────────

fn app_button(
    label: &str,
    is_running: bool,
    on_press: Option<Message>,
) -> Element<'_, Message, Theme, iced::Renderer> {
    let mut btn = Button::new(text(label).size(12)).padding([2, 6]).style(
        move |_theme: &Theme, status: Status| {
            let bg = if is_running {
                shared::colors::ACCENT
            } else if matches!(status, Status::Hovered) {
                Color::from_rgb(0.18, 0.18, 0.20)
            } else {
                Color::TRANSPARENT
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: shared::colors::FG,
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
                snap: false,
            }
        },
    );
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    btn.into()
}

// ── View ───────────────────────────────────────────────────────────────────

pub fn view(dock: &Dock) -> Element<'_, Message, Theme, iced::Renderer> {
    // Favorite app buttons (up to 10)
    let fav_buttons: Vec<Element<'_, Message, Theme, iced::Renderer>> = dock
        .favorites
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, app)| {
            let running = dock.running.iter().any(|id| {
                app.filename
                    .trim_end_matches(".desktop")
                    .eq_ignore_ascii_case(id)
            });
            app_button(&app.name, running, Some(Message::Launch(i)))
        })
        .collect();

    let extra_buttons: Vec<Element<'_, Message, Theme, iced::Renderer>> = dock
        .running
        .iter()
        .filter(|app_id| {
            !dock.favorites.iter().any(|fav| {
                fav.filename
                    .trim_end_matches(".desktop")
                    .eq_ignore_ascii_case(app_id)
            })
        })
        .map(|app_id| {
            let idx = dock.running.iter().position(|r| r == app_id).unwrap_or(0);
            app_button(app_id, true, Some(Message::Focus(idx)))
        })
        .collect();

    let separator: Element<'_, Message, Theme, iced::Renderer> = if extra_buttons.is_empty() {
        Space::new().width(0).into()
    } else {
        Space::new().width(8).into()
    };

    let content = row![
        row(fav_buttons).spacing(2),
        separator,
        row(extra_buttons).spacing(2),
        Space::new().width(Length::Fill),
    ]
    .padding([0, DOCK_PADDING as u16])
    .height(DOCK_HEIGHT)
    .align_y(iced::Alignment::Center);

    container(content)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(shared::colors::BG)),
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .into()
}

// ── Favorites scanning ─────────────────────────────────────────────────────

/// Scan `.desktop` files and return entries for common app categories.
pub fn scan_favorites() -> Vec<shared::apps::AppEntry> {
    let all_apps = shared::apps::scan_apps();
    let mut favs: Vec<shared::apps::AppEntry> = all_apps
        .into_values()
        .filter(|app| {
            app.categories.iter().any(|cat| {
                matches!(
                    cat.as_str(),
                    "Utility"
                        | "Development"
                        | "Office"
                        | "Network"
                        | "AudioVideo"
                        | "Graphics"
                        | "Game"
                )
            })
        })
        .collect();
    favs.sort_by(|a, b| a.name.cmp(&b.name));
    favs.truncate(10);
    favs
}
