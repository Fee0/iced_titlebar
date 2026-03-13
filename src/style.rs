//! Titlebar and window chrome styling.
//!
//! Centralizes colors for the titlebar bar, default button background, min/max hover, and close hover.

use iced::widget::button::{self, Status as ButtonStatus};
use iced::widget::container;
use iced::{Color, Theme};

/// Colors used for the titlebar and its buttons.
///
/// - `bar`: Background of the whole titlebar and default background of all three buttons.
/// - `button_hover`: Hover/pressed background for minimize and maximize buttons.
/// - `close_hover`: Hover/pressed background for the close button (typically red).
#[derive(Debug, Clone, Copy)]
pub struct TitlebarStyle {
    /// Background color for the titlebar and for all buttons in their default state.
    pub bar: Color,
    /// Hover/pressed background for minimize and maximize buttons.
    pub button_hover: Color,
    /// Hover/pressed background for the close button.
    pub close_hover: Color,
    /// Icon/text color used on the buttons (e.g. light gray on dark bar).
    pub icon: Color,
}

impl Default for TitlebarStyle {
    fn default() -> Self {
        Self {
            bar: Color::from_rgb8(30, 30, 30),
            button_hover: Color::from_rgb8(60, 60, 60),
            close_hover: Color::from_rgb8(232, 17, 35),
            icon: Color::from_rgb8(240, 240, 240),
        }
    }
}

/// Returns the container style for the titlebar (full bar background).
pub fn bar_container_style(style: &TitlebarStyle) -> container::Style {
    container::Style::default()
        .background(iced::Background::Color(style.bar))
}

/// Returns the button style for minimize and maximize: bar color by default, `button_hover` when hovered/pressed.
pub fn min_max_button_style(
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
pub fn close_button_style(
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
