//! Status bar — wlr-layer-shell top panel.

use chrono::Local;
use iced::widget::{Space, container, row, text};
use iced::{Element, Length, Theme};

use crate::shared;

// ── State ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Bar {
    pub clock: String,
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

// ── Update ─────────────────────────────────────────────────────────────────

pub fn update(bar: &mut Bar, msg: Message) {
    match msg {
        Message::Tick => {
            bar.clock = Local::now().format("%H:%M").to_string();
        }
    }
}

// ── View ───────────────────────────────────────────────────────────────────

pub fn view(bar: &Bar) -> Element<'_, Message, Theme, iced::Renderer> {
    let content = row![
        text(" driftshell ").size(13),
        Space::new().width(Length::Fill),
        text(&bar.clock).size(13),
    ]
    .padding([0, shared::BAR_PADDING as u16])
    .height(shared::BAR_HEIGHT);

    container(content)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(shared::colors::BG)),
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .into()
}
