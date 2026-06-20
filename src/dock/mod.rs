//! Dock — wlr-layer-shell bottom bar with favorite and running apps.

use iced::widget::button::{self, Button, Status};
use iced::widget::image::{self, Image};
use iced::widget::tooltip::{self, Tooltip};
use iced::widget::{Column, Space, container, row, scrollable, text};
use iced::{Border, Color, Element, Theme};

use crate::driftwm;
use crate::shared::{apps, colors, icons, style};

// ── State ──

#[derive(Debug, Clone, Default)]
pub struct Dock {
    pub favorites: Vec<apps::AppEntry>,
    pub running: Vec<String>,
    pub active: Option<String>,
}

// ── Messages ──

#[derive(Debug, Clone)]
pub enum Message {
    State(Vec<driftwm::ipc::WindowInfo>),
    Launch(usize),
    Focus(usize),
}

// ── Update ──

pub fn update(dock: &mut Dock, msg: Message) {
    match msg {
        Message::State(windows) => {
            dock.active = windows
                .iter()
                .find(|w| w.is_focused)
                .map(|w| w.app_id.clone());
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

// ── View helpers ──

fn app_icon<'a>(
    app_id: &'a str,
    app_name: &'a str,
    is_active: bool,
    is_running: bool,
    on_press: Option<Message>,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let icon: Element<'_, Message, Theme, iced::Renderer> = if let Some(path) = icons::find(app_id)
    {
        Image::new(image::Handle::from_path(path))
            .width(32.0)
            .height(32.0)
            .into()
    } else {
        text(app_name.chars().next().unwrap_or('?'))
            .size(style::FONT_M)
            .width(32.0)
            .height(32.0)
            .into()
    };

    let indicator = {
        let color = if is_active {
            colors::PRIMARY
        } else if is_running {
            colors::OUTLINE
        } else {
            Color::TRANSPARENT
        };
        container(Space::new().width(16.0).height(3.0)).style(move |_: &Theme| container::Style {
            background: Some(iced::Background::Color(color)),
            border: Border {
                radius: (2.0).into(),
                ..Default::default()
            },
            ..Default::default()
        })
    };

    let col = Column::new()
        .push(icon)
        .push(indicator)
        .spacing(2)
        .align_x(iced::Alignment::Center);

    let mut btn = Button::new(col)
        .padding([style::MARGIN_XS as u16; 2])
        .style(move |_theme: &Theme, status: Status| {
            let bg = if matches!(status, Status::Hovered) {
                colors::SURFACE_VARIANT
            } else {
                Color::TRANSPARENT
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: colors::ON_SURFACE,
                border: Border {
                    radius: style::IRADIUS_S.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
                snap: false,
            }
        });

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    Tooltip::new(
        btn,
        text(app_name).size(style::FONT_XS),
        tooltip::Position::Top,
    )
    .gap(4)
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(colors::SURFACE_VARIANT)),
        text_color: Some(colors::ON_SURFACE),
        border: Border {
            radius: style::IRADIUS_S.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    })
    .into()
}

// ── View ──

pub fn view(dock: &Dock) -> Element<'_, Message, Theme, iced::Renderer> {
    let fav_buttons: Vec<Element<'_, Message, Theme, iced::Renderer>> = dock
        .favorites
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, app)| {
            let app_id = app.filename.trim_end_matches(".desktop");
            let is_active = dock.active.as_deref() == Some(app_id);
            let is_running = dock.running.iter().any(|id| {
                app.filename
                    .trim_end_matches(".desktop")
                    .eq_ignore_ascii_case(id)
            });
            app_icon(
                app_id,
                &app.name,
                is_active,
                is_running,
                Some(Message::Launch(i)),
            )
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
            let is_active = dock.active.as_deref() == Some(app_id.as_str());
            app_icon(app_id, app_id, is_active, true, Some(Message::Focus(idx)))
        })
        .collect();

    let separator: Element<'_, Message, Theme, iced::Renderer> = if extra_buttons.is_empty() {
        Space::new().width(0).into()
    } else {
        Space::new().width(style::MARGIN_M).into()
    };

    let apps_row = row![
        row(fav_buttons).spacing(style::MARGIN_XS),
        separator,
        row(extra_buttons).spacing(style::MARGIN_XS),
    ]
    .padding([0, style::MARGIN_M as u16])
    .height(style::DOCK_HEIGHT)
    .align_y(iced::Alignment::Center);

    let content = scrollable(apps_row)
        .direction(scrollable::Direction::Horizontal(Default::default()))
        .height(style::DOCK_HEIGHT);

    container(content)
        .center_x(iced::Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(colors::SURFACE)),
            border: Border {
                radius: style::RADIUS_L.into(),
                width: 1.0,
                color: colors::OUTLINE,
            },
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .into()
}

// ── Favorites scanning ──

/// Scan `.desktop` files and return entries for common app categories.
pub fn scan_favorites() -> Vec<apps::AppEntry> {
    let all_apps = apps::scan_apps();
    let mut favs: Vec<apps::AppEntry> = all_apps
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
