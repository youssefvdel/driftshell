//! Root Iced Application — runs the bar via iced_layershell.

use iced::Element;
use iced::Task as Command;
use iced_layershell::reexport::*;
use iced_layershell::settings::{LayerShellSettings, StartMode};
use iced_layershell::{application, to_layer_message};

use crate::bar;
use crate::driftwm;

// ── Messages ───────────────────────────────────────────────────────────────

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Bar(bar::Message),
    Driftwm(driftwm::Message),
    Tick,
}

// ── App State ──────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct App {
    pub bar: bar::Bar,
    pub driftwm: driftwm::State,
}

// ── Public Entrypoint ──────────────────────────────────────────────────────

pub fn run() -> Result<(), iced_layershell::Error> {
    application(
        || (App::default(), schedule_tick()),
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

// ── Application Logic ──────────────────────────────────────────────────────

impl App {
    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Bar(msg) => {
                bar::update(&mut self.bar, msg);
                Command::none()
            }
            Message::Driftwm(msg) => {
                driftwm::update(&mut self.driftwm, msg);
                Command::none()
            }
            Message::Tick => {
                bar::update(&mut self.bar, bar::Message::Tick);
                schedule_tick()
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        bar::view(&self.bar).map(Message::Bar)
    }
}

fn schedule_tick() -> Command<Message> {
    Command::perform(
        async {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        },
        |_| Message::Tick,
    )
}
