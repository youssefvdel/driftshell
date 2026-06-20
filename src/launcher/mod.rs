//! App launcher — fuzzy-find and launch applications via driftwm IPC.

use iced::widget::{column, container, scrollable, text, text_input};
use iced::{Border, Element, Length, Theme};

use std::collections::HashMap;

use crate::shared;
use crate::shared::apps::AppEntry;

// ── State ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Launcher {
    pub visible: bool,
    pub query: String,
    pub results: Vec<(String, AppEntry)>,
    pub all_apps: Vec<(String, AppEntry)>,
    pub selected: Option<usize>,
}

impl Launcher {
    pub fn new(apps: HashMap<String, AppEntry>) -> Self {
        let mut all: Vec<_> = apps.into_iter().collect();
        all.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let results = all.clone();
        let mut s = Self {
            visible: false,
            query: String::new(),
            results,
            all_apps: all,
            selected: None,
        };
        s.filter();
        s
    }

    /// Filter results based on the current query.
    pub fn filter(&mut self) {
        if self.query.is_empty() {
            self.results = self.all_apps.clone();
        } else {
            let q = self.query.to_lowercase();
            self.results = self
                .all_apps
                .iter()
                .filter(|(_id, a)| {
                    a.name.to_lowercase().contains(&q)
                        || a.filename.to_lowercase().contains(&q)
                        || a.categories.iter().any(|c| c.to_lowercase().contains(&q))
                })
                .cloned()
                .collect();
        }
        self.selected = if self.results.is_empty() {
            None
        } else {
            Some(0)
        };
    }
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)] // Select/Launch/Close — keyboard nav planned
pub enum Message {
    Toggle,
    QueryChanged(String),
    Select(usize),
    Launch(usize),
    Close,
}

// ── Update ─────────────────────────────────────────────────────────────────

pub fn update(launcher: &mut Launcher, msg: &Message) {
    match msg {
        Message::Toggle => {
            launcher.visible = !launcher.visible;
            if launcher.visible {
                launcher.query.clear();
                launcher.filter();
                launcher.selected = Some(0);
            }
        }
        Message::QueryChanged(query) => {
            launcher.query = query.clone();
            launcher.filter();
        }
        Message::Select(idx) => {
            launcher.selected = Some(*idx);
        }
        Message::Launch(idx) => {
            if let Some((_id, app)) = launcher.results.get(*idx) {
                log::info!("launch: {} ({})", app.name, app.exec);
                if let Err(e) = std::process::Command::new("sh")
                    .args(["-c", &app.exec])
                    .spawn()
                {
                    log::error!("launch failed: {e}");
                }
            }
            launcher.visible = false;
        }
        Message::Close => {
            launcher.visible = false;
        }
    }
}

// ── View ───────────────────────────────────────────────────────────────────

pub fn view(launcher: &Launcher) -> Element<'_, Message, Theme, iced::Renderer> {
    let search_input = text_input("Search applications...", &launcher.query)
        .on_input(Message::QueryChanged)
        .padding(10)
        .size(16)
        .width(Length::Fill);

    let results_list: Element<'_, Message, Theme, iced::Renderer> = if launcher.results.is_empty() {
        text("No matching applications").size(14).into()
    } else {
        let items: Vec<Element<'_, Message, Theme, iced::Renderer>> = launcher
            .results
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, (_id, app))| {
                let is_selected = launcher.selected == Some(i);
                let prefix = if is_selected { "▸ " } else { "  " };
                let label = format!("{prefix}{}", app.name);
                let row = text(label).size(14);

                container(row)
                    .padding([4, 8])
                    .style(move |_theme: &Theme| {
                        if is_selected {
                            container::Style {
                                background: Some(iced::Background::Color(shared::colors::ACCENT)),
                                border: Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        } else {
                            container::Style::default()
                        }
                    })
                    .into()
            })
            .collect();

        scrollable(column(items).spacing(2))
            .height(Length::Fill)
            .into()
    };

    let content = column![search_input, results_list].spacing(8).padding(16);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(shared::colors::SURFACE)),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}
