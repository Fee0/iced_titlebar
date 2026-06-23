//! Shared titlebar types, resize handles, and layout helpers used by [crate::windows] and [crate::mac].

use crate::style::TitlebarStyle;
use iced::mouse::Interaction;
use iced::widget::{column, container, mouse_area, row};
use iced::{Element, Length};

/// Messages emitted by custom titlebars. Map them in your app to [iced::window] tasks.
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
pub const DEFAULT_TITLEBAR_HEIGHT: f32 = 29.0;

/// Width or height of edge resize strips in pixels.
pub const RESIZE_EDGE_SIZE: f32 = 5.0;

/// Size of corner resize handles (each side) in pixels.
pub const RESIZE_CORNER_SIZE: f32 = 5.0;

/// Draggable title strip: single press starts drag, double-click toggles maximize.
pub(crate) fn draggable_title_area<'a, Message, Theme>(
    title: Element<'a, Message, Theme, iced::Renderer>,
    to_message: &dyn Fn(TitlebarMessage) -> Message,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: container::Catalog + 'static,
{
    mouse_area(container(title).width(Length::Fill).height(Length::Fill))
        .on_press(to_message(TitlebarMessage::StartDrag))
        .on_double_click(to_message(TitlebarMessage::ToggleMaximize))
        .into()
}

/// Stacks `bar` above `content` and wraps the result in resize handles plus a border from `chrome`.
pub(crate) fn surround_with_resize_edges<'a, Message, Theme>(
    bar: Element<'a, Message, Theme, iced::Renderer>,
    content: Element<'a, Message, Theme, iced::Renderer>,
    resize_edge_size: Option<f32>,
    border_width: Option<f32>,
    chrome: TitlebarStyle,
    to_resize: impl Fn(iced::window::Direction) -> Message + 'a,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: container::Catalog + iced::widget::text::Catalog + 'static,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    let edge = resize_edge_size.unwrap_or(RESIZE_EDGE_SIZE);
    let visual_border = border_width.unwrap_or(edge);
    let style = chrome;
    let inner = column![bar, content]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    let with_handles = resize_handles_with_sizes(inner, to_resize, edge, edge);
    container(with_handles)
        .style(move |_theme| {
            iced::widget::container::Style::default().border(
                iced::Border::default()
                    .width(visual_border)
                    .color(style.border),
            )
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn resize_cursor_for(direction: iced::window::Direction) -> Interaction {
    use iced::window::Direction;
    match direction {
        Direction::North | Direction::South => Interaction::ResizingVertically,
        Direction::East | Direction::West => Interaction::ResizingHorizontally,
        Direction::NorthEast | Direction::SouthWest => Interaction::ResizingDiagonallyUp,
        Direction::NorthWest | Direction::SouthEast => Interaction::ResizingDiagonallyDown,
    }
}

pub(crate) fn resize_handles_with_sizes<'a, Message, Theme>(
    content: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
    to_message: impl Fn(iced::window::Direction) -> Message + 'a,
    edge_size: f32,
    corner_size: f32,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + iced::widget::text::Catalog + 'static,
{
    let resize_region = |direction: iced::window::Direction, width: Length, height: Length| {
        use iced::widget::text;
        container(
            mouse_area(
                container(text(" ").size(1))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .interaction(resize_cursor_for(direction))
            .on_press(to_message(direction)),
        )
        .width(width)
        .height(height)
    };

    let nw = resize_region(
        iced::window::Direction::NorthWest,
        Length::Fixed(corner_size),
        Length::Fixed(corner_size),
    );
    let n = resize_region(
        iced::window::Direction::North,
        Length::Fill,
        Length::Fixed(edge_size),
    );
    let ne = resize_region(
        iced::window::Direction::NorthEast,
        Length::Fixed(corner_size),
        Length::Fixed(corner_size),
    );

    let w = resize_region(
        iced::window::Direction::West,
        Length::Fixed(edge_size),
        Length::Fill,
    );
    let e = resize_region(
        iced::window::Direction::East,
        Length::Fixed(edge_size),
        Length::Fill,
    );

    let sw = resize_region(
        iced::window::Direction::SouthWest,
        Length::Fixed(corner_size),
        Length::Fixed(corner_size),
    );
    let s = resize_region(
        iced::window::Direction::South,
        Length::Fill,
        Length::Fixed(edge_size),
    );
    let se = resize_region(
        iced::window::Direction::SouthEast,
        Length::Fixed(corner_size),
        Length::Fixed(corner_size),
    );

    let top_row = row![nw, n, ne].spacing(0);
    let mid_row = row![w, content.into(), e].spacing(0);
    let bot_row = row![sw, s, se].spacing(0);

    column![top_row, mid_row, bot_row]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Wraps content with invisible resize handles on all four edges and four corners.
///
/// Map each handle’s message to `window::drag_resize(window_id, direction)`.
///
/// For a titlebar + content layout with configurable edge size, use [crate::windows::TitleBarWindows::with_content]
/// or [crate::mac::TitleBarMac::with_content].
pub fn resize_handles<'a, Message, Theme>(
    content: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
    to_message: impl Fn(iced::window::Direction) -> Message + 'a,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + iced::widget::text::Catalog + 'static,
{
    resize_handles_with_sizes(content, to_message, RESIZE_EDGE_SIZE, RESIZE_CORNER_SIZE)
}
