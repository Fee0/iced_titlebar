//! Custom titlebar widget for iced applications with decorations disabled.
//!
//! Emits [TitlebarMessage] that the app maps to [iced::window] tasks in its update function.

use crate::style;
use iced::widget::{button, container, mouse_area, row, svg, text};
use iced::{Element, Length};
use iced::widget::svg::Handle as SvgHandle;

/// Messages emitted by the custom titlebar widget.
/// Map these in your app's update to the corresponding [iced::window] tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitlebarMessage {
    /// User pressed on the draggable title area — run `window::drag(window_id)`.
    StartDrag,
    /// User clicked minimize — run `window::minimize(window_id, true)`.
    Minimize,
    /// User clicked maximize/restore or double-clicked the title bar — run `window::toggle_maximize(window_id)`.
    ToggleMaximize,
    /// User clicked close — run `window::close(window_id)`.
    Close,
}

/// Default height of the titlebar in pixels.
pub const DEFAULT_TITLEBAR_HEIGHT: f32 = 32.0;

/// Builds a custom titlebar row: draggable title area (full width except buttons) + minimize, maximize, close buttons.
///
/// The entire bar except the three buttons is draggable. Double-clicking the title area toggles maximize/restore.
///
/// * `title` — Text shown centered in the title area.
/// * `to_message` — Converts [TitlebarMessage] into your app's `Message` (e.g. `Message::Titlebar`).
///
/// In your update, handle the titlebar message and return the matching task:
/// * `StartDrag` → `window::drag(window_id)`
/// * `Minimize` → `window::minimize(window_id, true)`
/// * `ToggleMaximize` → `window::toggle_maximize(window_id)` (button or double-click on bar)
/// * `Close` → `window::close(window_id)`
pub fn titlebar<'a, Message>(
    title: impl ToString,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let title_str = title.to_string();
    let draggable = container(
        mouse_area(
            container(text(title_str).size(14))
                .padding(iced::Padding::from([8, 12]))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .on_press(to_message(TitlebarMessage::StartDrag))
        .on_double_click(to_message(TitlebarMessage::ToggleMaximize)),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let min_icon = svg(minimize_handle())
        .width(16)
        .height(16);

    let max_icon = svg(maximize_handle())
        .width(16)
        .height(16);

    let close_icon = svg(close_handle())
        .width(14)
        .height(14);

    let min_btn = button(min_icon)
        .on_press(to_message(TitlebarMessage::Minimize))
        .style(|theme, status| style::min_max_button_style(&style::TitlebarStyle::default(), theme, status))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let max_btn = button(max_icon)
        .on_press(to_message(TitlebarMessage::ToggleMaximize))
        .style(|theme, status| style::min_max_button_style(&style::TitlebarStyle::default(), theme, status))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let close_btn = button(close_icon)
        .on_press(to_message(TitlebarMessage::Close))
        .style(|theme, status| style::close_button_style(&style::TitlebarStyle::default(), theme, status))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let row = row![draggable, min_btn, max_btn, close_btn]
        .spacing(0)
        .height(DEFAULT_TITLEBAR_HEIGHT)
        .align_y(iced::Alignment::Center);

    container(row)
        .style(|_theme| style::bar_container_style(&style::TitlebarStyle::default()))
        .height(DEFAULT_TITLEBAR_HEIGHT)
        .width(Length::Fill)
        .into()
}

/// SVG handle for the minimize icon: a single horizontal line.
fn minimize_handle() -> SvgHandle {
    // 10x10 viewBox with a centered horizontal line
    const MINIMIZE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
  <line x1="2" y1="5" x2="8" y2="5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
</svg>"#;
    SvgHandle::from_memory(MINIMIZE_SVG.as_bytes().to_vec())
}

/// SVG handle for the maximize/restore icon: two overlapping squares.
fn maximize_handle() -> SvgHandle {
    const MAXIMIZE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
  <rect x="2" y="2" width="5" height="5" fill="none" stroke="currentColor" stroke-width="1"/>
  <rect x="3" y="3" width="5" height="5" fill="none" stroke="currentColor" stroke-width="1"/>
</svg>"#;
    SvgHandle::from_memory(MAXIMIZE_SVG.as_bytes().to_vec())
}

/// SVG handle for the close icon: an X.
fn close_handle() -> SvgHandle {
    const CLOSE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
  <line x1="2" y1="2" x2="8" y2="8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
  <line x1="8" y1="2" x2="2" y2="8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
</svg>"#;
    SvgHandle::from_memory(CLOSE_SVG.as_bytes().to_vec())
}

