//! Shared theme and common widgets for driftshell — Catppuccin Mocha.

pub mod apps;
pub mod icons;

pub mod colors {
    use iced::Color;

    // Catppuccin Mocha
    pub const PRIMARY: Color = Color::from_rgb(0.541, 0.725, 0.980); // #89b4fa Blue
    pub const ON_PRIMARY: Color = Color::from_rgb(0.118, 0.118, 0.180); // #1e1e2e Base
    #[allow(dead_code)]
    pub const SECONDARY: Color = Color::from_rgb(0.796, 0.651, 0.969); // #cba6f7 Mauve
    #[allow(dead_code)]
    pub const TERTIARY: Color = Color::from_rgb(0.580, 0.886, 0.835); // #94e2d5 Teal

    pub const SURFACE: Color = Color::from_rgb(0.118, 0.118, 0.180); // #1e1e2e Base
    pub const ON_SURFACE: Color = Color::from_rgb(0.804, 0.839, 0.957); // #cdd6f4 Text
    pub const SURFACE_VARIANT: Color = Color::from_rgb(0.192, 0.196, 0.267); // #313244 Surface0
    pub const ON_SURFACE_VARIANT: Color = Color::from_rgb(0.424, 0.439, 0.525); // #6c7086 Overlay0

    pub const OUTLINE: Color = Color::from_rgb(0.271, 0.275, 0.361); // #45475a Surface1
    pub const ERROR: Color = Color::from_rgb(0.953, 0.545, 0.659); // #f38ba8 Red
    pub const HOVER: Color = Color::from_rgb(0.651, 0.890, 0.631); // #a6e3a1 Green
}

/// Noctalia-style constants — radii, spacing, font sizes.
pub mod style {
    /// Bar / dock height (default comfortable horizontal).
    pub const BAR_HEIGHT: f32 = 37.0;
    pub const DOCK_HEIGHT: f32 = 48.0;

    /// Capsule (pill) height — ~82 % of bar height.
    pub const CAPSULE_HEIGHT: f32 = 31.0;
    /// Capsule radius.
    pub const CAPSULE_RADIUS: f32 = 16.0;

    /// Container radii.
    pub const RADIUS_M: f32 = 12.0;
    pub const RADIUS_L: f32 = 16.0;

    /// Input / button radii.
    pub const IRADIUS_S: f32 = 8.0;

    /// Spacing / margin.
    pub const MARGIN_XS: f32 = 4.0;
    pub const MARGIN_S: f32 = 6.0;
    pub const MARGIN_M: f32 = 9.0;
    pub const MARGIN_L: f32 = 13.0;

    /// Font sizes.
    pub const FONT_XXS: f32 = 9.0;
    pub const FONT_XS: f32 = 10.0;
    pub const FONT_S: f32 = 11.0;
    pub const FONT_M: f32 = 13.0;
    pub const FONT_L: f32 = 16.0;
}
