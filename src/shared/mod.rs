//! Shared theme and common widgets for driftshell.

pub mod apps;

pub mod colors {
    use iced::Color;
    pub const BG: Color = Color::from_rgb(0.08, 0.08, 0.10);
    pub const FG: Color = Color::from_rgb(0.92, 0.92, 0.95);
    pub const ACCENT: Color = Color::from_rgb(0.30, 0.60, 1.0);
    pub const SURFACE: Color = Color::from_rgb(0.12, 0.12, 0.14);
}

pub const BAR_HEIGHT: f32 = 36.0;
pub const BAR_PADDING: f32 = 8.0;
