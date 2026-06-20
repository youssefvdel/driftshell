//! Status bar — wlr-layer-shell top panel.

use chrono::Local;
use iced::widget::button::{self, Button, Status};
use iced::widget::{Space, container, row, text};
use iced::{Color, Element, Font, Length, Theme};

use crate::driftwm;
use crate::shared;

// ── State ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Bar {
    pub clock: String,
    pub workspaces: Vec<driftwm::Workspace>,
    pub tray_services: Vec<String>,
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Workspaces(Vec<super::driftwm::Workspace>),
    FocusWorkspace(String),
    TrayUpdate(Vec<String>),
    ToggleLauncher,
    ToggleSettings,
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
        Message::TrayUpdate(services) => {
            bar.tray_services = services;
        }
        Message::ToggleLauncher | Message::ToggleSettings => {
            // Handled by app-level update
        }
    }
}

// ── View helpers ────────────────────────────────────────────────────────────

fn pill_button<'a>(
    label: &'a str,
    is_active: bool,
    msg: Message,
) -> Element<'a, Message, Theme, iced::Renderer> {
    Button::new(
        text(label)
            .size(shared::style::FONT_XS)
            .font(Font::with_name("JetBrains Mono")),
    )
    .padding([2, shared::style::MARGIN_S as u16])
    .style(move |_theme: &Theme, status: Status| {
        let bg = if is_active {
            shared::colors::PRIMARY
        } else {
            Color::TRANSPARENT
        };
        let text_color = if is_active {
            shared::colors::ON_PRIMARY
        } else if matches!(status, Status::Hovered) {
            shared::colors::HOVER
        } else {
            shared::colors::ON_SURFACE
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color,
            border: iced::Border {
                radius: if is_active {
                    shared::style::IRADIUS_S.into()
                } else {
                    0.0.into()
                },
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    })
    .on_press(msg)
    .into()
}

fn workspace_button(ws: &driftwm::Workspace) -> Element<'_, Message, Theme, iced::Renderer> {
    let label = text(&ws.name)
        .size(shared::style::FONT_XS)
        .font(Font::with_name("JetBrains Mono"));
    Button::new(label)
        .padding([2, shared::style::MARGIN_S as u16])
        .style(move |_theme: &Theme, status: Status| {
            let bg = if ws.active {
                shared::colors::PRIMARY
            } else {
                Color::TRANSPARENT
            };
            let text_color = if ws.active {
                shared::colors::ON_PRIMARY
            } else if matches!(status, Status::Hovered) {
                shared::colors::HOVER
            } else {
                shared::colors::ON_SURFACE
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color,
                border: iced::Border {
                    radius: if ws.active {
                        shared::style::IRADIUS_S.into()
                    } else {
                        0.0.into()
                    },
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

fn capsule<'a>(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, iced::Renderer>>,
) -> Element<'a, Message, Theme, iced::Renderer> {
    container(
        row(children)
            .align_y(iced::Alignment::Center)
            .spacing(shared::style::MARGIN_S),
    )
    .height(shared::style::CAPSULE_HEIGHT)
    .align_y(iced::Alignment::Center)
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(shared::colors::SURFACE_VARIANT)),
        border: iced::Border {
            radius: shared::style::CAPSULE_RADIUS.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    })
    .into()
}

// ── View ───────────────────────────────────────────────────────────────────

pub fn view(bar: &Bar) -> Element<'_, Message, Theme, iced::Renderer> {
    // ── Right section: tray services + clock ────────────────────────────────
    let mut right_children: Vec<Element<'_, Message, Theme, iced::Renderer>> = bar
        .tray_services
        .iter()
        .map(|s| {
            text(s)
                .size(shared::style::FONT_XXS)
                .font(Font::with_name("JetBrains Mono"))
                .into()
        })
        .collect();
    right_children.push(
        text(&bar.clock)
            .size(shared::style::FONT_XS)
            .font(Font::with_name("JetBrains Mono"))
            .into(),
    );

    // ── Left section: Apps button + workspace buttons + Settings button ─────
    let mut left_children: Vec<Element<'_, Message, Theme, iced::Renderer>> = Vec::new();
    left_children.push(pill_button("  Apps", false, Message::ToggleLauncher));
    left_children.extend(bar.workspaces.iter().map(workspace_button));
    left_children.push(pill_button("  Sets", false, Message::ToggleSettings));

    let content = row![
        capsule(left_children),
        Space::new().width(Length::Fill),
        capsule(right_children),
    ]
    .padding([0, shared::style::MARGIN_S as u16])
    .height(shared::style::BAR_HEIGHT)
    .align_y(iced::Alignment::Center);

    container(content)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(shared::colors::SURFACE)),
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .into()
}
