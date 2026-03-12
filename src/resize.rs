//! Resize handles for borderless iced windows.
//!
//! When window decorations are disabled, use these handles so the user can resize by dragging
//! the four edges (North, South, East, West) and four corners (NorthWest, NorthEast, SouthWest, SouthEast).

use iced::widget::{container, mouse_area, row, column, text};
use iced::{Element, Length};

/// Width or height of edge resize strips in pixels.
pub const RESIZE_EDGE_SIZE: f32 = 5.0;

/// Size of corner resize handles (each side) in pixels.
pub const RESIZE_CORNER_SIZE: f32 = 5.0;

/// Wraps content with invisible resize handles on all four edges and four corners.
///
/// On left press, each handle emits a message with the corresponding [iced::window::Direction],
/// which the app should map to `window::drag_resize(window_id, direction)` in update.
///
/// * `content` — The main UI (e.g. a column with titlebar + body).
/// * `to_message` — Converts [iced::window::Direction] into your app's `Message`.
pub fn resize_handles<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    to_message: impl Fn(iced::window::Direction) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let edge_size = RESIZE_EDGE_SIZE;
    let corner_size = RESIZE_CORNER_SIZE;

    // Use size(1) — cosmic-text panics on line height 0; handles stay effectively invisible at 1px.
    let resize_region = |direction: iced::window::Direction, width: Length, height: Length| {
        container(
            mouse_area(
                container(text(" ").size(1))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .on_press(to_message(direction)),
        )
        .width(width)
        .height(height)
    };

    let nw = resize_region(iced::window::Direction::NorthWest, Length::Fixed(corner_size), Length::Fixed(corner_size));
    let n  = resize_region(iced::window::Direction::North, Length::Fill, Length::Fixed(edge_size));
    let ne = resize_region(iced::window::Direction::NorthEast, Length::Fixed(corner_size), Length::Fixed(corner_size));

    let w  = resize_region(iced::window::Direction::West, Length::Fixed(edge_size), Length::Fill);
    let e  = resize_region(iced::window::Direction::East, Length::Fixed(edge_size), Length::Fill);

    let sw = resize_region(iced::window::Direction::SouthWest, Length::Fixed(corner_size), Length::Fixed(corner_size));
    let s  = resize_region(iced::window::Direction::South, Length::Fill, Length::Fixed(edge_size));
    let se = resize_region(iced::window::Direction::SouthEast, Length::Fixed(corner_size), Length::Fixed(corner_size));

    let top_row = row![nw, n, ne].spacing(0);
    let mid_row = row![w, content.into(), e].spacing(0);
    let bot_row = row![sw, s, se].spacing(0);

    column![top_row, mid_row, bot_row]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
