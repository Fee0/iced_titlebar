//! Windows-style titlebar (icons on the right) for iced with decorations disabled.
//!
//! Emits [TitlebarMessage](crate::common::TitlebarMessage) that the app maps to [iced::window] tasks.
//! Resize helpers live in [crate::common].

pub use crate::common::{
    resize_handles, TitlebarMessage, DEFAULT_TITLEBAR_HEIGHT, RESIZE_CORNER_SIZE, RESIZE_EDGE_SIZE,
};

use crate::common::{draggable_title_area, surround_with_resize_edges};
use crate::style::{self, TitleAlignment};
use iced::alignment::Horizontal;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, container, row, svg, text};
use iced::{Alignment, Element, Length};

/// Default width in logical pixels for each minimize / maximize / close button hit target.
pub const TITLEBAR_WINDOWS_CONTROL_WIDTH: f32 = 45.0;

/// Custom titlebar widget: draggable title area + minimize, maximize, close buttons.
///
/// Build with [titlebar_windows](titlebar_windows)(title), then chain [on_message](TitleBarWindows::on_message), [style](TitleBarWindows::style), [height](TitleBarWindows::height),
/// [title_alignment](TitleBarWindows::title_alignment), [resize_edge](TitleBarWindows::resize_edge), [maximized](TitleBarWindows::maximized), [icon_spacing](TitleBarWindows::icon_spacing). Call [.into()](Into::into) to get an `Element`,
/// or [with_content](TitleBarWindows::with_content) to stack the bar with content and wrap everything in resize handles.
/// You must call [on_message](TitleBarWindows::on_message) for the bar to be interactive.
/// Pass the current window maximized state via [maximized](TitleBarWindows::maximized) so the middle button shows the correct icon (maximize vs restore).
pub struct TitleBarWindows<'a, Message, Theme = iced::Theme> {
    /// Title text shown in the draggable area.
    pub title: String,
    /// Visual style (bar/button colors, icon color).
    pub style: style::TitlebarStyle,
    /// Height of the bar in pixels.
    pub height: f32,
    /// Horizontal alignment of the title text (left, center, right).
    pub title_alignment: TitleAlignment,
    /// Whether the window is currently maximized. When true, the middle button shows the restore icon; otherwise the maximize icon.
    /// Track this in app state (e.g. toggle on [ToggleMaximize](TitlebarMessage::ToggleMaximize)) or use [iced::window::is_maximized](https://docs.rs/iced/latest/iced/window/fn.is_maximized.html).
    pub is_maximized: bool,
    /// Optional resize edge thickness (in pixels) for integrated resize handles.
    /// When None, the default [RESIZE_EDGE_SIZE] is used.
    pub resize_edge_size: Option<f32>,
    /// Horizontal spacing between the minimize, maximize, and close buttons only (not the title).
    pub icon_spacing: f32,
    /// Callback to convert [TitlebarMessage] into your app's `Message`. Required for interaction.
    pub on_message: Option<Box<dyn Fn(TitlebarMessage) -> Message + 'a>>,
    _theme: std::marker::PhantomData<Theme>,
}

impl<'a, Message, Theme> std::fmt::Debug for TitleBarWindows<'a, Message, Theme> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TitleBarWindows")
            .field("title", &self.title)
            .field("style", &self.style)
            .field("height", &self.height)
            .field("title_alignment", &self.title_alignment)
            .field("is_maximized", &self.is_maximized)
            .field("icon_spacing", &self.icon_spacing)
            .field("on_message", &self.on_message.is_some())
            .finish()
    }
}

/// Creates a new [TitleBarWindows] with the given title and default style/height. Call [.on_message()](TitleBarWindows::on_message) and then [.into()](Into::into) to build the element.
///
/// # Example
///
/// ```
/// # use iced_custom_titlebar::{titlebar_windows, TitlebarMessage};
/// # use iced::Element;
/// # #[derive(Clone)]
/// # enum Message { Titlebar(TitlebarMessage) }
/// let _bar: Element<'_, Message> = titlebar_windows("My App").on_message(Message::Titlebar).into();
/// ```
pub fn titlebar_windows<Message, Theme>(title: impl ToString) -> TitleBarWindows<'static, Message, Theme> {
    TitleBarWindows {
        title: title.to_string(),
        style: style::TitlebarStyle::default(),
        height: DEFAULT_TITLEBAR_HEIGHT,
        title_alignment: TitleAlignment::default(),
        is_maximized: false,
        resize_edge_size: None,
        icon_spacing: 0.0,
        on_message: None,
        _theme: std::marker::PhantomData,
    }
}

impl<'a, Message, Theme> TitleBarWindows<'a, Message, Theme> {
    /// Sets the callback that maps [TitlebarMessage] to your app's `Message`. Required for drag/button interaction.
    pub fn on_message<'b, F>(self, f: F) -> TitleBarWindows<'b, Message, Theme>
    where
        F: Fn(TitlebarMessage) -> Message + 'b,
    {
        TitleBarWindows {
            title: self.title,
            style: self.style,
            height: self.height,
            title_alignment: self.title_alignment,
            is_maximized: self.is_maximized,
            resize_edge_size: self.resize_edge_size,
            icon_spacing: self.icon_spacing,
            on_message: Some(Box::new(f)),
            _theme: std::marker::PhantomData,
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
    /// Used by [with_content](TitleBarWindows::with_content) when wrapping content in resize handles.
    pub fn resize_edge(mut self, size: f32) -> Self {
        self.resize_edge_size = Some(size.max(0.0));
        self
    }

    /// Sets the horizontal alignment of the title text (left, center, right).
    pub fn title_alignment(mut self, a: TitleAlignment) -> Self {
        self.title_alignment = a;
        self
    }

    /// Sets whether the window is currently maximized. When true, the middle button shows the restore icon (two overlapping squares); otherwise the maximize icon (single square).
    /// Track this in your app state (e.g. flip on [ToggleMaximize](TitlebarMessage::ToggleMaximize)) or sync from [iced::window::is_maximized](https://docs.rs/iced/latest/iced/window/fn.is_maximized.html).
    pub fn maximized(mut self, value: bool) -> Self {
        self.is_maximized = value;
        self
    }

    /// Sets horizontal spacing between the three window-control icons (minimize, maximize, close).
    pub fn icon_spacing(mut self, spacing: f32) -> Self {
        self.icon_spacing = spacing.clamp(0.0, 64.0);
        self
    }
}

impl<'a, Message, Theme> From<TitleBarWindows<'a, Message, Theme>> for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    fn from(value: TitleBarWindows<'a, Message, Theme>) -> Self {
        let to_message = value.on_message.expect(
            "titlebar_windows: on_message must be set before converting to Element (e.g. titlebar_windows(\"App\").on_message(Message::Titlebar).into())",
        );
        build_titlebar_windows_element(
            value.title,
            value.style,
            value.height,
            value.title_alignment,
            value.is_maximized,
            value.icon_spacing,
            to_message,
        )
    }
}

impl<'a, Message, Theme> TitleBarWindows<'a, Message, Theme>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    /// Builds a layout with this titlebar on top of `content`, wrapped in resize handles.
    ///
    /// The returned element includes the outer border (using the titlebar style's border color and the same thickness as the resize edge). The resize edge thickness is taken from [resize_edge](TitleBarWindows::resize_edge) if set,
    /// otherwise it falls back to [RESIZE_EDGE_SIZE].
    pub fn with_content(
        self,
        content: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
        to_resize: impl Fn(iced::window::Direction) -> Message + 'a,
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let resize_edge_size = self.resize_edge_size;
        let chrome = self.style;
        let bar: Element<'a, Message, Theme, iced::Renderer> = self.into();
        surround_with_resize_edges(
            bar,
            content.into(),
            resize_edge_size,
            chrome,
            to_resize,
        )
    }
}

/// Builds a custom titlebar element. Used by [From] and [titlebar_windows_with_style].
fn build_titlebar_windows_element<'a, Message, Theme>(
    title_str: String,
    style: style::TitlebarStyle,
    height: f32,
    title_alignment: TitleAlignment,
    is_maximized: bool,
    icon_spacing: f32,
    to_message: Box<dyn Fn(TitlebarMessage) -> Message + 'a>,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    let draggable = draggable_title_area(title_str, style, title_alignment, &*to_message);

    let s_min = style;
    let s_max = style;
    let s_close = style;
    let s_bar = style;

    let min_icon = container(
        svg(minimize_handle()).width(10).height(10).style(
            move |_theme, _status| svg::Style {
                color: Some(s_min.icon),
            },
        ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Alignment::Center);

    let max_handle = if is_maximized {
        restore_handle()
    } else {
        maximize_handle()
    };
    let max_icon = container(
        svg(max_handle)
            .width(10)
            .height(10)
            .style(move |_theme, _status| svg::Style {
                color: Some(s_max.icon),
            }),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Alignment::Center);

    let close_icon = container(
        svg(close_handle()).width(10).height(10).style(
            move |_theme, _status| svg::Style {
                color: Some(s_close.icon),
            },
        ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Alignment::Center);

    let min_btn = button(min_icon)
        .on_press(to_message(TitlebarMessage::Minimize))
        .style(move |theme, status| style::min_max_button_style(&s_min, theme, status))
        .width(TITLEBAR_WINDOWS_CONTROL_WIDTH)
        .height(Length::Fill);

    let max_btn = button(max_icon)
        .on_press(to_message(TitlebarMessage::ToggleMaximize))
        .style(move |theme, status| style::min_max_button_style(&s_max, theme, status))
        .width(TITLEBAR_WINDOWS_CONTROL_WIDTH)
        .height(Length::Fill);

    let close_btn = button(close_icon)
        .on_press(to_message(TitlebarMessage::Close))
        .style(move |theme, status| style::close_button_style(&s_close, theme, status))
        .width(TITLEBAR_WINDOWS_CONTROL_WIDTH)
        .height(Length::Fill);

    let controls = row![min_btn, max_btn, close_btn]
        .spacing(icon_spacing)
        .height(Length::Fill)
        .align_y(Alignment::Center);

    let row = row![draggable, controls]
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
/// Prefer the builder form: `titlebar_windows(title).style(style).maximized(is_maximized).on_message(to_message).into()`.
pub fn titlebar_windows_with_style<'a, Message, Theme>(
    title: impl ToString,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
    style: style::TitlebarStyle,
    title_alignment: TitleAlignment,
    is_maximized: bool,
    icon_spacing: f32,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    build_titlebar_windows_element(
        title.to_string(),
        style,
        DEFAULT_TITLEBAR_HEIGHT,
        title_alignment,
        is_maximized,
        icon_spacing,
        Box::new(to_message),
    )
}

/// SVG handle for the minimize icon: a single horizontal line 10px wide (crisp 10×10 viewBox, 1px stroke).
fn minimize_handle() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/windows/minimize.svg"
        ))
        .to_vec(),
    )
}

/// SVG handle for the maximize icon: single square (expand to full screen). Shown when window is not maximized.
fn maximize_handle() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/windows/maximize.svg"
        ))
        .to_vec(),
    )
}

/// SVG handle for the restore icon: two overlapping squares (restore down). Shown when window is maximized.
fn restore_handle() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/windows/restore.svg"
        ))
        .to_vec(),
    )
}

/// SVG handle for the close icon: an X (crisp 10×10, 1px stroke, butt caps to match reference).
fn close_handle() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/windows/close.svg"
        ))
        .to_vec(),
    )
}
