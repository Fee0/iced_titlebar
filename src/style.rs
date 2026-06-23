//! Titlebar and window chrome styling.
//!
//! Centralizes colors for the titlebar bar, default button background, min/max hover, close hover, and icon color.

use iced::widget::button::{self, Status as ButtonStatus};
use iced::widget::container;
use iced::Color;

/// Horizontal alignment of the title text inside the titlebar draggable area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleAlignment {
    /// Title aligned to the left.
    Left,
    /// Title centered (default).
    #[default]
    Center,
    /// Title aligned to the right.
    Right,
}

impl std::fmt::Display for TitleAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitleAlignment::Left => write!(f, "Left"),
            TitleAlignment::Center => write!(f, "Center"),
            TitleAlignment::Right => write!(f, "Right"),
        }
    }
}

/// Style for the titlebar and its buttons: bar and button hover colors, icon color, title font color.
///
/// - `bar`: Background of the whole titlebar and default background of all three buttons.
/// - `button_hover`: Hover/pressed background for minimize and maximize buttons.
/// - `close_hover`: Hover/pressed background for the close button (typically red).
/// - `icon`: Color used for the SVG window-control icons (minimize, maximize, close) and any button text. SVGs use `currentColor` so they inherit this.
/// - `font_color`: Color of the title text in the draggable area.
#[derive(Debug, Clone, Copy)]
pub struct TitlebarStyle {
    /// Background color for the titlebar and for all buttons in their default state.
    pub bar: Color,
    /// Hover/pressed background for minimize and maximize buttons.
    pub button_hover: Color,
    /// Hover/pressed background for the close button.
    pub close_hover: Color,
    /// Color for the SVG icons (minimize, maximize, close) and button text. SVGs use `currentColor` so they inherit this.
    pub icon: Color,
    /// Color of the border around the app (the resize-edge / drag region). The titlebar draws this border when using [TitleBarWindows::with_content](crate::windows::TitleBarWindows::with_content) or [TitleBarMac::with_content](crate::mac::TitleBarMac::with_content).
    pub border: Color,
    /// Color of the title text in the draggable area.
    pub font_color: Color,
}

impl Default for TitlebarStyle {
    fn default() -> Self {
        Self::preset(TitlebarStylePreset::Dark)
    }
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
                bar: Color::from_rgb8(0, 0, 0),
                button_hover: Color::from_rgb8(60, 60, 60),
                close_hover: Color::from_rgb8(232, 17, 35),
                icon: Color::from_rgb8(255, 255, 255),
                border: Color::from_rgb8(52, 53, 56),
                font_color: Color::from_rgb8(255, 255, 255),
            },
            TitlebarStylePreset::Light => TitlebarStyle {
                bar: Color::from_rgb8(255, 255, 255),
                button_hover: Color::from_rgb8(220, 220, 220),
                close_hover: Color::from_rgb8(232, 17, 35),
                icon: Color::from_rgb8(0, 0, 0),
                border: Color::from_rgb8(160, 160, 160),
                font_color: Color::from_rgb8(0, 0, 0),
            },
        }
    }
}

/// Returns the container style for the titlebar (background only; no border).
pub fn bar_container_style(style: &TitlebarStyle) -> container::Style {
    container::Style::default().background(iced::Background::Color(style.bar))
}

/// Returns the button style for minimize and maximize: bar color by default, `button_hover` when hovered/pressed.
pub fn min_max_button_style<Theme>(
    style: &TitlebarStyle,
    _theme: &Theme,
    status: ButtonStatus,
) -> button::Style {
    use button::Status::*;

    let mut s = button::Style::default();
    s.background = Some(iced::Background::Color(style.bar));
    s.border = iced::Border::default().width(0.0);
    s.text_color = style.icon;

    if matches!(status, Hovered | Pressed) {
        s.background = Some(iced::Background::Color(style.button_hover));
    }

    s
}

/// Returns the button style for the close button: bar color by default, `close_hover` when hovered/pressed.
pub fn close_button_style<Theme>(
    style: &TitlebarStyle,
    _theme: &Theme,
    status: ButtonStatus,
) -> button::Style {
    use button::Status::*;

    let mut s = button::Style::default();
    s.background = Some(iced::Background::Color(style.bar));
    s.border = iced::Border::default().width(0.0);
    s.text_color = style.icon;

    if matches!(status, Hovered | Pressed) {
        s.background = Some(iced::Background::Color(style.close_hover));
    }

    s
}

/// Flat traffic-light buttons: bar background only, no hover or pressed highlight.
pub fn traffic_light_button_style<Theme>(
    style: &TitlebarStyle,
    _theme: &Theme,
    _status: ButtonStatus,
) -> button::Style {
    let mut s = button::Style::default();
    s.background = Some(iced::Background::Color(style.bar));
    s.border = iced::Border::default().width(0.0);
    s.text_color = style.icon;
    s
}
