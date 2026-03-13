//! Custom titlebar widget for iced applications with decorations disabled.
//!
//! Emits [TitlebarMessage] that the app maps to [iced::window] tasks in its update function.
//! Also provides resize handles for borderless windows (edges and corners).

use crate::style::{self, TitleAlignment};
use iced::mouse::Interaction;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Length};

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

/// Custom titlebar widget: draggable title area + minimize, maximize, close buttons.
///
/// Build with [titlebar](titlebar)(title), then chain [on_message](Titlebar::on_message), [style](Titlebar::style), [height](Titlebar::height),
/// [title_alignment](Titlebar::title_alignment), [resize_edge](Titlebar::resize_edge). Call [.into()](Into::into) to get an `Element`,
/// or [with_content](Titlebar::with_content) to stack the bar with content and wrap everything in resize handles.
/// You must call [on_message](Titlebar::on_message) for the bar to be interactive.
pub struct Titlebar<'a, Message> {
    /// Title text shown in the draggable area.
    pub title: String,
    /// Visual style (bar/button colors, icon color).
    pub style: style::TitlebarStyle,
    /// Height of the bar in pixels.
    pub height: f32,
    /// Horizontal alignment of the title text (left, center, right).
    pub title_alignment: TitleAlignment,
    /// Optional resize edge thickness (in pixels) for integrated resize handles.
    /// When None, the default [RESIZE_EDGE_SIZE] is used.
    pub resize_edge_size: Option<f32>,
    /// Callback to convert [TitlebarMessage] into your app's `Message`. Required for interaction.
    pub on_message: Option<Box<dyn Fn(TitlebarMessage) -> Message + 'a>>,
}

impl<'a, Message> std::fmt::Debug for Titlebar<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Titlebar")
            .field("title", &self.title)
            .field("style", &self.style)
            .field("height", &self.height)
            .field("title_alignment", &self.title_alignment)
            .field("on_message", &self.on_message.is_some())
            .finish()
    }
}

/// Creates a new [Titlebar] with the given title and default style/height. Call [.on_message()](Titlebar::on_message) and then [.into()](Into::into) to build the element.
///
/// # Example
///
/// ```
/// # use iced_custom_titlebar::{titlebar, TitlebarMessage};
/// # enum Message { Titlebar(TitlebarMessage) }
/// let bar = titlebar("My App").on_message(Message::Titlebar).into();
/// ```
pub fn titlebar<Message>(title: impl ToString) -> Titlebar<'static, Message> {
    Titlebar {
        title: title.to_string(),
        style: style::TitlebarStyle::default(),
        height: DEFAULT_TITLEBAR_HEIGHT,
        title_alignment: TitleAlignment::default(),
        resize_edge_size: None,
        on_message: None,
    }
}

impl<'a, Message> Titlebar<'a, Message> {
    /// Sets the callback that maps [TitlebarMessage] to your app's `Message`. Required for drag/button interaction.
    pub fn on_message<'b, F>(self, f: F) -> Titlebar<'b, Message>
    where
        F: Fn(TitlebarMessage) -> Message + 'b,
    {
        Titlebar {
            title: self.title,
            style: self.style,
            height: self.height,
            title_alignment: self.title_alignment,
            resize_edge_size: self.resize_edge_size,
            on_message: Some(Box::new(f)),
        }
    }

    /// Sets the full [TitlebarStyle] (bar/button colors, border, icon color, title alignment).
    pub fn style(mut self, s: style::TitlebarStyle) -> Self {
        self.style = s;
        self
    }

    /// Sets the height of the titlebar in pixels.
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    /// Sets the resize edge/corner thickness (in pixels) for integrated resize handles.
    /// Used by [with_content](Titlebar::with_content) when wrapping content in resize handles.
    pub fn resize_edge(mut self, size: f32) -> Self {
        self.resize_edge_size = Some(size.max(0.0));
        self
    }

    /// Sets the horizontal alignment of the title text (left, center, right).
    pub fn title_alignment(mut self, a: TitleAlignment) -> Self {
        self.title_alignment = a;
        self
    }
}

impl<'a, Message> From<Titlebar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    fn from(value: Titlebar<'a, Message>) -> Self {
        let to_message = value.on_message.expect(
            "titlebar: on_message must be set before converting to Element (e.g. titlebar(\"App\").on_message(Message::Titlebar).into())",
        );
        build_titlebar_element(
            value.title,
            value.style,
            value.height,
            value.title_alignment,
            to_message,
        )
    }
}

impl<'a, Message> Titlebar<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    /// Builds a layout with this titlebar on top of `content`, wrapped in resize handles.
    ///
    /// The resize edge thickness is taken from [resize_edge](Titlebar::resize_edge) if set,
    /// otherwise it falls back to [RESIZE_EDGE_SIZE].
    pub fn with_content(
        self,
        content: impl Into<Element<'a, Message>>,
        to_resize: impl Fn(iced::window::Direction) -> Message + 'a,
    ) -> Element<'a, Message> {
        let edge = self.resize_edge_size.unwrap_or(RESIZE_EDGE_SIZE);
        let bar: Element<'a, Message> = self.into();

        let inner = column![bar, content.into()]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill);

        resize_handles_with_sizes(inner, to_resize, edge, edge)
    }
}

/// Builds a custom titlebar element. Used by [From] and [titlebar_with_style].
fn build_titlebar_element<'a, Message>(
    title_str: String,
    style: style::TitlebarStyle,
    height: f32,
    title_alignment: TitleAlignment,
    to_message: Box<dyn Fn(TitlebarMessage) -> Message + 'a>,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let title_align = to_iced_alignment(title_alignment);

    let draggable = container(
        mouse_area(
            container(text(title_str).size(14))
                .padding(iced::Padding::from([8, 12]))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(title_align)
                .align_y(Alignment::Center),
        )
        .on_press(to_message(TitlebarMessage::StartDrag))
        .on_double_click(to_message(TitlebarMessage::ToggleMaximize)),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let s_min = style;
    let s_max = style;
    let s_close = style;
    let s_bar = style;

    let min_icon = svg(minimize_handle())
        .width(16)
        .height(16)
        .style(move |_theme, _status| iced::widget::svg::Style {
            color: Some(s_min.icon),
        });

    let max_icon = svg(maximize_handle())
        .width(16)
        .height(16)
        .style(move |_theme, _status| iced::widget::svg::Style {
            color: Some(s_max.icon),
        });

    let close_icon = svg(close_handle())
        .width(14)
        .height(14)
        .style(move |_theme, _status| iced::widget::svg::Style {
            color: Some(s_close.icon),
        });

    let min_btn = button(min_icon)
        .on_press(to_message(TitlebarMessage::Minimize))
        .style(move |theme, status| style::min_max_button_style(&s_min, theme, status))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let max_btn = button(max_icon)
        .on_press(to_message(TitlebarMessage::ToggleMaximize))
        .style(move |theme, status| style::min_max_button_style(&s_max, theme, status))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let close_btn = button(close_icon)
        .on_press(to_message(TitlebarMessage::Close))
        .style(move |theme, status| style::close_button_style(&s_close, theme, status))
        .padding(4)
        .width(46)
        .height(Length::Fill);

    let row = row![draggable, min_btn, max_btn, close_btn]
        .spacing(0)
        .height(height)
        .align_y(Alignment::Center);

    container(row)
        .style(move |_theme| style::bar_container_style(&s_bar))
        .height(height)
        .width(Length::Fill)
        .into()
}

/// Builds a custom titlebar with the given style (convenience wrapper around the builder).
///
/// Prefer the builder form: `titlebar(title).style(style).on_message(to_message).into()`.
pub fn titlebar_with_style<'a, Message>(
    title: impl ToString,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
    style: style::TitlebarStyle,
    title_alignment: TitleAlignment,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    build_titlebar_element(
        title.to_string(),
        style,
        DEFAULT_TITLEBAR_HEIGHT,
        title_alignment,
        Box::new(to_message),
    )
}

fn to_iced_alignment(a: TitleAlignment) -> Alignment {
    match a {
        TitleAlignment::Left => Alignment::Start,
        TitleAlignment::Center => Alignment::Center,
        TitleAlignment::Right => Alignment::End,
    }
}

/// SVG handle for the minimize icon: a single horizontal line.
fn minimize_handle() -> SvgHandle {
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

// ---------------------------------------------------------------------------
// Resize handles (for borderless windows)
// ---------------------------------------------------------------------------

/// Width or height of edge resize strips in pixels.
pub const RESIZE_EDGE_SIZE: f32 = 5.0;

/// Size of corner resize handles (each side) in pixels.
pub const RESIZE_CORNER_SIZE: f32 = 5.0;

fn resize_cursor_for(direction: iced::window::Direction) -> Interaction {
    use iced::window::Direction;
    match direction {
        Direction::North | Direction::South => Interaction::ResizingVertically,
        Direction::East | Direction::West => Interaction::ResizingHorizontally,
        Direction::NorthEast | Direction::SouthWest => Interaction::ResizingDiagonallyUp,
        Direction::NorthWest | Direction::SouthEast => Interaction::ResizingDiagonallyDown,
    }
}

fn resize_handles_with_sizes<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    to_message: impl Fn(iced::window::Direction) -> Message + 'a,
    edge_size: f32,
    corner_size: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let resize_region = |direction: iced::window::Direction, width: Length, height: Length| {
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
/// When window decorations are disabled, use this so the user can resize by dragging
/// the edges (North, South, East, West) and corners (NorthWest, NorthEast, SouthWest, SouthEast).
/// On left press, each handle emits a message with the corresponding [iced::window::Direction];
/// map it to `window::drag_resize(window_id, direction)` in your update.
///
/// For a titlebar + content layout with configurable edge size, use [Titlebar::with_content] instead.
pub fn resize_handles<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    to_message: impl Fn(iced::window::Direction) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    resize_handles_with_sizes(content, to_message, RESIZE_EDGE_SIZE, RESIZE_CORNER_SIZE)
}
