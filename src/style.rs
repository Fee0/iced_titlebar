//! Titlebar and window chrome styling.

use iced::Color;
use iced::widget::button::{self, Status as ButtonStatus};

/// Style for the titlebar and its buttons.
///
/// The titlebar background is transparent by default (`background: None`) so it inherits the
/// active theme's window background. Set `background` to `Some(color)` to paint a solid bar.
/// The title area accepts any iced `Element`, so text color and layout are the caller's responsibility.
#[derive(Debug, Clone, Copy)]
pub struct TitlebarStyle {
    /// Background color of the titlebar strip. `None` = transparent (inherits window background).
    pub background: Option<Color>,
    /// Hover/pressed background for minimize and maximize buttons.
    pub button_hover: Color,
    /// Hover/pressed background for the close button.
    pub close_hover: Color,
    /// Color for the SVG icons (minimize, maximize, close). SVGs use `currentColor` so they inherit this.
    pub icon: Color,
    /// Color of the border around the app (the resize-edge / drag region). Used by [TitleBarWindows::with_content](crate::windows::TitleBarWindows::with_content) and [TitleBarMac::with_content](crate::mac::TitleBarMac::with_content).
    pub border: Color,
}

impl Default for TitlebarStyle {
    fn default() -> Self {
        Self::preset(TitlebarStylePreset::Dark)
    }
}

/// Which side of the titlebar the window-control buttons (close/min/max) appear on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlsSide {
    /// Controls on the left (macOS default).
    Left,
    /// Controls on the right (Windows default).
    Right,
}

/// Built-in style variants for the titlebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitlebarStylePreset {
    /// Dark titlebar (default).
    #[default]
    Dark,
    /// Light titlebar.
    Light,
}

impl std::fmt::Display for TitlebarStylePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitlebarStylePreset::Dark => write!(f, "Dark"),
            TitlebarStylePreset::Light => write!(f, "Light"),
        }
    }
}

impl From<TitlebarStylePreset> for TitlebarStyle {
    fn from(preset: TitlebarStylePreset) -> Self {
        Self::preset(preset)
    }
}

impl TitlebarStyle {
    /// Returns the style for a built-in preset.
    pub fn preset(preset: TitlebarStylePreset) -> Self {
        match preset {
            TitlebarStylePreset::Dark => TitlebarStyle {
                background: None,
                button_hover: Color::from_rgb8(60, 60, 60),
                close_hover: Color::from_rgb8(232, 17, 35),
                icon: Color::from_rgb8(255, 255, 255),
                border: Color::from_rgb8(52, 53, 56),
            },
            TitlebarStylePreset::Light => TitlebarStyle {
                background: None,
                button_hover: Color::from_rgb8(220, 220, 220),
                close_hover: Color::from_rgb8(232, 17, 35),
                icon: Color::from_rgb8(0, 0, 0),
                border: Color::from_rgb8(160, 160, 160),
            },
        }
    }
}

/// Returns the button style for minimize and maximize: transparent by default, `button_hover` when hovered/pressed.
pub fn min_max_button_style<Theme>(
    style: &TitlebarStyle,
    _theme: &Theme,
    status: ButtonStatus,
) -> button::Style {
    use button::Status::*;

    let background =
        matches!(status, Hovered | Pressed).then_some(iced::Background::Color(style.button_hover));
    button::Style {
        background,
        border: iced::Border::default().width(0.0),
        text_color: style.icon,
        ..Default::default()
    }
}

/// Returns the button style for the close button: transparent by default, `close_hover` when hovered/pressed.
pub fn close_button_style<Theme>(
    style: &TitlebarStyle,
    _theme: &Theme,
    status: ButtonStatus,
) -> button::Style {
    use button::Status::*;

    let background =
        matches!(status, Hovered | Pressed).then_some(iced::Background::Color(style.close_hover));
    button::Style {
        background,
        border: iced::Border::default().width(0.0),
        text_color: style.icon,
        ..Default::default()
    }
}

/// Flat traffic-light buttons: transparent background, no hover or pressed highlight.
pub fn traffic_light_button_style<Theme>(
    style: &TitlebarStyle,
    _theme: &Theme,
    _status: ButtonStatus,
) -> button::Style {
    button::Style {
        border: iced::Border::default().width(0.0),
        text_color: style.icon,
        ..Default::default()
    }
}
