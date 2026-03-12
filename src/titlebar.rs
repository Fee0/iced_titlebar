//! Custom titlebar widget for iced applications with decorations disabled.
//!
//! Emits [TitlebarMessage] that the app maps to [iced::window] tasks in its update function.

use iced::widget::{button, container, mouse_area, row, text};
use iced::{Element, Length};

/// Messages emitted by the custom titlebar widget.
/// Map these in your app's update to the corresponding [iced::window] tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitlebarMessage {
    /// User pressed on the draggable title area — run `window::drag(window_id)`.
    StartDrag,
    /// User clicked minimize — run `window::minimize(window_id, true)`.
    Minimize,
    /// User clicked maximize/restore — run `window::toggle_maximize(window_id)`.
    ToggleMaximize,
    /// User clicked close — run `window::close(window_id)`.
    Close,
}

/// Default height of the titlebar in pixels.
pub const DEFAULT_TITLEBAR_HEIGHT: f32 = 32.0;

/// Builds a custom titlebar row: draggable title area + minimize, maximize, close buttons.
///
/// * `title` — Text shown in the title area (draggable region).
/// * `to_message` — Converts [TitlebarMessage] into your app's `Message` (e.g. `Message::Titlebar`).
///
/// In your update, handle the titlebar message and return the matching task:
/// * `StartDrag` → `window::drag(window_id)`
/// * `Minimize` → `window::minimize(window_id, true)`
/// * `ToggleMaximize` → `window::toggle_maximize(window_id)`
/// * `Close` → `window::close(window_id)`
pub fn titlebar<'a, Message>(
    title: impl ToString,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let title_str = title.to_string();
    let draggable = container(
        mouse_area(
            container(text(title_str).size(14))
                .padding(iced::Padding::from([8, 12]))
                .center_y(Length::Fill),
        )
        .on_press(to_message(TitlebarMessage::StartDrag)),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let min_btn = button(text("−").size(16))
        .on_press(to_message(TitlebarMessage::Minimize))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let max_btn = button(text("□").size(14))
        .on_press(to_message(TitlebarMessage::ToggleMaximize))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let close_btn = button(text("×").size(18))
        .on_press(to_message(TitlebarMessage::Close))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let row = row![draggable, min_btn, max_btn, close_btn]
        .spacing(0)
        .height(DEFAULT_TITLEBAR_HEIGHT)
        .align_y(iced::Alignment::Center);

    container(row)
        .height(DEFAULT_TITLEBAR_HEIGHT)
        .width(Length::Fill)
        .into()
}
