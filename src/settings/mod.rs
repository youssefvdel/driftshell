//! Settings window — layer-shell overlay for driftwm config.
//!
//! Form fields for cursor, background, etc.
//! Save writes config.toml + triggers driftwm reload-config.

use iced::widget::{
    button, column, container, pick_list, row, scrollable, text, text_input, toggler,
};
use iced::{Border, Element, Length, Theme};

use crate::driftwm::config::{self, DriftwmConfig};
use crate::shared::{colors, style};

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
    DecorationFontChanged(String),
    DecorationFontSizeChanged(String),
    DecorationFontWeightChanged(String),
    BarOpacityChanged(String),
    CapsuleShowToggled(bool),
    AnimationSpeedChanged(String),
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
        Message::DecorationFontChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let dec = cfg.decorations.get_or_insert_with(Default::default);
                let tb = dec.title_bar.get_or_insert_with(Default::default);
                tb.font = Some(val.clone());
            }
        }
        Message::DecorationFontSizeChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let dec = cfg.decorations.get_or_insert_with(Default::default);
                let tb = dec.title_bar.get_or_insert_with(Default::default);
                tb.font_size = val.parse().ok();
            }
        }
        Message::DecorationFontWeightChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let dec = cfg.decorations.get_or_insert_with(Default::default);
                let tb = dec.title_bar.get_or_insert_with(Default::default);
                tb.font_weight = Some(val.clone());
            }
        }
        Message::BarOpacityChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let bar = cfg.bar.get_or_insert_with(Default::default);
                bar.opacity = val.parse().ok();
            }
        }
        Message::CapsuleShowToggled(val) => {
            if let Some(cfg) = &mut settings.config {
                let bar = cfg.bar.get_or_insert_with(Default::default);
                bar.capsule_show = Some(*val);
            }
        }
        Message::AnimationSpeedChanged(val) => {
            if let Some(cfg) = &mut settings.config {
                let general = cfg.general.get_or_insert_with(Default::default);
                general.animation_speed = val.parse().ok();
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
    row![
        text(label)
            .width(120)
            .size(style::FONT_S)
            .color(colors::ON_SURFACE),
        input
            .style(
                |_theme: &Theme, _status: text_input::Status| text_input::Style {
                    background: iced::Background::Color(colors::SURFACE_VARIANT),
                    border: Border {
                        radius: style::IRADIUS_S.into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
                    },
                    icon: colors::ON_SURFACE_VARIANT,
                    placeholder: colors::ON_SURFACE_VARIANT,
                    value: colors::ON_SURFACE,
                    selection: colors::PRIMARY,
                }
            )
            .padding([6, 10]),
    ]
    .spacing(style::MARGIN_S)
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

    let dec_font = cfg
        .decorations
        .as_ref()
        .and_then(|d| d.title_bar.as_ref())
        .and_then(|t| t.font.as_deref())
        .unwrap_or("");
    let dec_font_size = cfg
        .decorations
        .as_ref()
        .and_then(|d| d.title_bar.as_ref())
        .and_then(|t| t.font_size)
        .map(|s| s.to_string())
        .unwrap_or_default();
    let dec_font_weight = cfg
        .decorations
        .as_ref()
        .and_then(|d| d.title_bar.as_ref())
        .and_then(|t| t.font_weight.as_deref())
        .unwrap_or("");

    let bar_opacity = cfg
        .bar
        .as_ref()
        .and_then(|b| b.opacity)
        .map(|s| s.to_string())
        .unwrap_or_default();
    let capsule_show = cfg
        .bar
        .as_ref()
        .and_then(|b| b.capsule_show)
        .unwrap_or(false);
    let animation_speed = cfg
        .general
        .as_ref()
        .and_then(|g| g.animation_speed)
        .map(|s| s.to_string())
        .unwrap_or_default();

    let content = column![
        // ── Header ──
        text("Settings")
            .size(style::FONT_L)
            .color(colors::ON_SURFACE),
        text("driftwm configuration")
            .size(style::FONT_XS)
            .color(colors::ON_SURFACE_VARIANT),
        // ── Cursor ──
        text("Cursor")
            .size(style::FONT_XS)
            .color(colors::ON_SURFACE_VARIANT),
        input_row(
            "Theme:",
            text_input("e.g. Adwaita, Bibata", cursor_theme).on_input(Message::CursorThemeChanged),
        ),
        input_row(
            "Size:",
            text_input("e.g. 24", &cursor_size).on_input(Message::CursorSizeChanged),
        ),
        // ── Background ──
        text("Background")
            .size(style::FONT_XS)
            .color(colors::ON_SURFACE_VARIANT),
        row![
            text("Type:")
                .width(120)
                .size(style::FONT_S)
                .color(colors::ON_SURFACE),
            pick_list(&["solid", "gradient", "shader"][..], Some(bg_type), |v| {
                Message::BackgroundTypeChanged(v.to_string())
            })
            .style(
                |_theme: &Theme, _status: pick_list::Status| pick_list::Style {
                    background: iced::Background::Color(colors::SURFACE_VARIANT),
                    text_color: colors::ON_SURFACE,
                    placeholder_color: colors::ON_SURFACE_VARIANT,
                    handle_color: colors::ON_SURFACE_VARIANT,
                    border: Border {
                        radius: style::IRADIUS_S.into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
                    },
                }
            ),
        ]
        .spacing(style::MARGIN_S)
        .align_y(iced::Alignment::Center),
        input_row(
            "Path:",
            text_input("/path/to/file", bg_path).on_input(Message::BackgroundPathChanged),
        ),
        // ── Decorations ──
        text("Decorations")
            .size(style::FONT_XS)
            .color(colors::ON_SURFACE_VARIANT),
        input_row(
            "Title bar font:",
            text_input("e.g. Sans, monospace", dec_font).on_input(Message::DecorationFontChanged),
        ),
        input_row(
            "Title bar font size:",
            text_input("e.g. 12", &dec_font_size).on_input(Message::DecorationFontSizeChanged),
        ),
        input_row(
            "Title bar font weight:",
            text_input("e.g. normal, bold", dec_font_weight)
                .on_input(Message::DecorationFontWeightChanged),
        ),
        // ── Bar ──
        text("Bar")
            .size(style::FONT_XS)
            .color(colors::ON_SURFACE_VARIANT),
        input_row(
            "Opacity:",
            text_input("0.0 - 1.0", &bar_opacity).on_input(Message::BarOpacityChanged),
        ),
        row![
            text("Show capsules:")
                .width(120)
                .size(style::FONT_S)
                .color(colors::ON_SURFACE),
            toggler(capsule_show).on_toggle(|v| Message::CapsuleShowToggled(v)),
        ]
        .spacing(style::MARGIN_S)
        .align_y(iced::Alignment::Center),
        // ── General ──
        text("General")
            .size(style::FONT_XS)
            .color(colors::ON_SURFACE_VARIANT),
        input_row(
            "Animation speed:",
            text_input("0.5 - 2.0", &animation_speed).on_input(Message::AnimationSpeedChanged),
        ),
        // ── Feedback ──
        if let Some(fb) = &settings.feedback {
            text(fb)
                .size(style::FONT_S)
                .color(if fb.starts_with("Save") {
                    colors::ERROR
                } else {
                    colors::PRIMARY
                })
        } else {
            text("").size(0)
        },
        // ── Save ──
        button(text("Save").size(style::FONT_M))
            .padding([8, 24])
            .on_press(Message::Save)
            .style(|_theme: &Theme, status: button::Status| button::Style {
                background: Some(iced::Background::Color(match status {
                    button::Status::Hovered => colors::HOVER,
                    _ => colors::PRIMARY,
                })),
                text_color: colors::ON_PRIMARY,
                border: Border {
                    radius: style::IRADIUS_S.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
                snap: false,
            }),
    ]
    .spacing(style::MARGIN_M)
    .padding(style::MARGIN_L as u16);

    let scrollable = scrollable(container(content).width(Length::Fill));

    container(scrollable)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(colors::SURFACE)),
            border: Border {
                radius: style::RADIUS_M.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
