//! Status bar — wlr-layer-shell top panel.

use chrono::Local;
use iced::widget::button::{self, Button, Status};
use iced::widget::{Space, container, row, text};
use iced::{Color, Element, Length, Theme};

use crate::driftwm;
use crate::shared;

// ── State ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Bar {
    pub clock: String,
    pub workspaces: Vec<driftwm::Workspace>,
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)] // ToggleLauncher wired when bar button added
pub enum Message {
    Tick,
    Workspaces(Vec<super::driftwm::Workspace>),
    FocusWorkspace(String),
    ToggleLauncher,
}

// ── Update ─────────────────────────────────────────────────────────────────

pub fn update(bar: &mut Bar, msg: Message) {
    match msg {
        Message::Tick => {
            bar.clock = Local::now().format("%H:%M").to_string();
        }
        Message::Workspaces(workspaces) => {
            bar.workspaces = workspaces;
        }
        Message::FocusWorkspace(app_id) => {
            if let Err(e) = driftwm::ipc::run_action(&format!("focus-window {app_id}")) {
                log::error!("focus workspace: {e}");
            }
        }
        Message::ToggleLauncher => {
            // Handled by app-level update; this arm exists for completeness
        }
    }
}

// ── View ───────────────────────────────────────────────────────────────────

fn workspace_button(ws: &driftwm::Workspace) -> Element<'_, Message, Theme, iced::Renderer> {
    let label = text(&ws.name).size(12);
    Button::new(label)
        .padding([2, 6])
        .style(move |_theme: &Theme, status: Status| {
            let bg = if ws.active {
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
        })
        .on_press(Message::FocusWorkspace(ws.name.clone()))
        .into()
}

pub fn view(bar: &Bar) -> Element<'_, Message, Theme, iced::Renderer> {
    let content = row![
        text("  driftshell").size(13),
        Space::new().width(4),
        row(bar
            .workspaces
            .iter()
            .map(workspace_button)
            .collect::<Vec<_>>(),)
        .spacing(2),
        Space::new().width(Length::Fill),
        text(&bar.clock).size(13),
    ]
    .padding([0, shared::BAR_PADDING as u16])
    .height(shared::BAR_HEIGHT)
    .align_y(iced::Alignment::Center);

    container(content)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(shared::colors::BG)),
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .into()
}
