//! Settings window — layer-shell overlay for driftwm config.
//!
//! Form fields for cursor, background, etc.
//! Save writes config.toml + triggers driftwm reload-config.

use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Element, Length, Theme};

use crate::driftwm::config::{self, DriftwmConfig};

// ── State ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Settings {
    pub visible: bool,
    /// The live config being edited. Loaded on open, serialized on save.
    pub config: Option<DriftwmConfig>,
    /// Feedback message shown after save attempt.
    pub feedback: Option<String>,
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    Open(Option<DriftwmConfig>),
    Close,
    CursorThemeChanged(String),
    CursorSizeChanged(String),
    BackgroundTypeChanged(String),
    BackgroundPathChanged(String),
    Save,
}

// ── Update ─────────────────────────────────────────────────────────────────

pub fn update(settings: &mut Settings, msg: &Message) {
    match msg {
        Message::Open(cfg) => {
            settings.visible = true;
            settings.config = cfg.clone();
            settings.feedback = None;
        }
        Message::Close => settings.visible = false,
        Message::CursorThemeChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let cursor = cfg.cursor.get_or_insert_with(Default::default);
                cursor.theme = Some(val.clone());
            }
        }
        Message::CursorSizeChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let cursor = cfg.cursor.get_or_insert_with(Default::default);
                cursor.size = val.parse().ok();
            }
        }
        Message::BackgroundTypeChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let bg = cfg.background.get_or_insert_with(Default::default);
                bg.bg_type = Some(val.clone());
            }
        }
        Message::BackgroundPathChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let bg = cfg.background.get_or_insert_with(Default::default);
                bg.path = Some(val.clone());
            }
        }
        Message::Save => {
            if let Some(cfg) = &settings.config {
                match config::write(cfg) {
                    Ok(()) => {
                        let _ = crate::driftwm::ipc::run_action("reload-config");
                        settings.feedback = Some("Saved. Config reloaded.".to_string());
                    }
                    Err(e) => settings.feedback = Some(format!("Save failed: {e}")),
                }
            }
        }
    }
}

// ── View ───────────────────────────────────────────────────────────────────

fn input_row<'a>(
    label: &'a str,
    input: iced::widget::text_input::TextInput<'a, Message, Theme, iced::Renderer>,
) -> Element<'a, Message, Theme, iced::Renderer> {
    row![text(label).width(120).size(14), input,]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .into()
}

pub fn view(settings: &Settings) -> Element<'_, Message, Theme, iced::Renderer> {
    let cfg = match &settings.config {
        Some(c) => c,
        None => {
            return container(text("No config loaded").size(16))
                .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill)
                .into();
        }
    };

    let cursor_theme = cfg
        .cursor
        .as_ref()
        .and_then(|c| c.theme.as_deref())
        .unwrap_or("");
    let cursor_size = cfg
        .cursor
        .as_ref()
        .and_then(|c| c.size)
        .map(|s| s.to_string())
        .unwrap_or_default();
    let bg_type = cfg
        .background
        .as_ref()
        .and_then(|b| b.bg_type.as_deref())
        .unwrap_or("shader");
    let bg_path = cfg
        .background
        .as_ref()
        .and_then(|b| b.path.as_deref())
        .unwrap_or("");

    let content = column![
        // ── Header ──
        text("Settings").size(20),
        text("driftwm configuration")
            .size(12)
            .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
        // ── Cursor ──
        text("Cursor").size(16),
        input_row(
            "Theme:",
            text_input("e.g. Adwaita, Bibata", cursor_theme).on_input(Message::CursorThemeChanged),
        ),
        input_row(
            "Size:",
            text_input("e.g. 24", &cursor_size).on_input(Message::CursorSizeChanged),
        ),
        // ── Background ──
        text("Background").size(16),
        row![
            text("Type:").width(120).size(14),
            pick_list(&["solid", "gradient", "shader"][..], Some(bg_type), |v| {
                Message::BackgroundTypeChanged(v.to_string())
            },),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        input_row(
            "Path:",
            text_input("/path/to/file", bg_path).on_input(Message::BackgroundPathChanged),
        ),
        // ── Feedback ──
        if let Some(fb) = &settings.feedback {
            text(fb).size(13).color(if fb.starts_with("Save") {
                iced::Color::from_rgb(0.8, 0.2, 0.2)
            } else {
                iced::Color::from_rgb(0.3, 0.8, 0.3)
            })
        } else {
            text("").size(0)
        },
        // ── Save ──
        button(text("Save").size(14))
            .padding([8, 24])
            .on_press(Message::Save),
    ]
    .spacing(12)
    .padding(24);

    let scrollable = scrollable(container(content).width(Length::Fill));

    container(scrollable)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(super::shared::colors::BG)),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
