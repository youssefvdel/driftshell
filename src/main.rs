//! driftshell — Rust Wayland shell for driftwm.
//!
//! Components:
//! - Bar: wlr-layer-shell top panel
//! - Dock: wlr-layer-shell bottom bar with favorite and running apps
//! - Launcher: overlay app launcher (TBD)
//! - Settings: XDG toplevel config window (TBD)
//! - Wallpaper: background via config.toml (TBD)

mod app;
mod bar;
mod dock;
mod driftwm;
mod launcher;
mod settings;
mod shared;
mod wallpaper;

use clap::Parser;

#[derive(Parser)]
#[command(name = "driftshell", version, about = "Rust Wayland shell for driftwm")]
struct Cli;

fn main() -> Result<(), iced_layershell::Error> {
    env_logger::init();
    let _cli = Cli::parse();

    log::info!("starting driftshell v{}", env!("CARGO_PKG_VERSION"));

    app::run()
}
