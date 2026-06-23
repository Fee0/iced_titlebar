//! Windows-style titlebar (icons on the right) for iced with decorations disabled.
//!
//! Emits [TitlebarMessage](crate::common::TitlebarMessage) that the app maps to [iced::window] tasks.
//! Resize helpers live in [crate::common].

pub use crate::common::{
    DEFAULT_TITLEBAR_HEIGHT, RESIZE_CORNER_SIZE, RESIZE_EDGE_SIZE, TitlebarMessage, resize_handles,
};

use crate::common::{draggable_title_area, surround_with_resize_edges};
use crate::style::{self, ControlsSide};
use iced::alignment::Horizontal;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, container, row, svg};
use iced::{Alignment, Element, Length};

/// Default width in logical pixels for each minimize / maximize / close button hit target.
pub const TITLEBAR_WINDOWS_CONTROL_WIDTH: f32 = 45.0;

/// Custom titlebar widget: draggable title area + minimize, maximize, close buttons.
///
/// Build with [titlebar_windows](titlebar_windows)(title), then chain [on_message](TitleBarWindows::on_message),
/// [style](TitleBarWindows::style), [height](TitleBarWindows::height), [resize_edge](TitleBarWindows::resize_edge),
/// [maximized](TitleBarWindows::maximized), [icon_spacing](TitleBarWindows::icon_spacing). Call [.into()](Into::into)
/// to get an `Element`, or [with_content](TitleBarWindows::with_content) to stack the bar with content and wrap
/// everything in resize handles. You must call [on_message](TitleBarWindows::on_message) for the bar to be interactive.
pub struct TitleBarWindows<'a, Message, Theme = iced::Theme> {
    /// Element shown in the draggable title area. Can be any iced widget (text, row, image, …).
    pub title: Element<'a, Message, Theme, iced::Renderer>,
    /// Visual style (bar/button colors, icon color).
    pub style: style::TitlebarStyle,
    /// Height of the bar in pixels.
    pub height: f32,
    /// Whether the window is currently maximized. When true, the middle button shows the restore icon; otherwise the maximize icon.
    pub is_maximized: bool,
    /// Optional resize edge thickness (in pixels) for integrated resize handles.
    pub resize_edge_size: Option<f32>,
    /// Horizontal spacing between the minimize, maximize, and close buttons only (not the title).
    pub icon_spacing: f32,
    /// Which side of the bar the controls appear on. Defaults to [ControlsSide::Right].
    pub controls_side: ControlsSide,
    /// Callback to convert [TitlebarMessage] into your app's `Message`. Required for interaction.
    pub on_message: Option<Box<dyn Fn(TitlebarMessage) -> Message + 'a>>,
    _theme: std::marker::PhantomData<Theme>,
}

impl<'a, Message, Theme> std::fmt::Debug for TitleBarWindows<'a, Message, Theme> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TitleBarWindows")
            .field("title", &"<Element>")
            .field("style", &self.style)
            .field("height", &self.height)
            .field("is_maximized", &self.is_maximized)
            .field("icon_spacing", &self.icon_spacing)
            .field("controls_side", &self.controls_side)
            .field("on_message", &self.on_message.is_some())
            .finish()
    }
}

/// Creates a new [TitleBarWindows] with the given title element and default style/height.
/// Pass any iced `Element` (or anything that converts `Into<Element>`) as the title.
/// Call [.on_message()](TitleBarWindows::on_message) and then [.into()](Into::into) to build the element.
pub fn titlebar_windows<'a, Message, Theme>(
    title: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
) -> TitleBarWindows<'a, Message, Theme> {
    TitleBarWindows {
        title: title.into(),
        style: style::TitlebarStyle::default(),
        height: DEFAULT_TITLEBAR_HEIGHT,
        is_maximized: false,
        resize_edge_size: None,
        icon_spacing: 0.0,
        controls_side: ControlsSide::Right,
        on_message: None,
        _theme: std::marker::PhantomData,
    }
}

impl<'a, Message, Theme> TitleBarWindows<'a, Message, Theme> {
    /// Sets the callback that maps [TitlebarMessage] to your app's `Message`. Required for drag/button interaction.
    pub fn on_message(mut self, f: impl Fn(TitlebarMessage) -> Message + 'a) -> Self {
        self.on_message = Some(Box::new(f));
        self
    }

    /// Sets the full [TitlebarStyle] (bar/button colors, border, icon color).
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

    /// Sets whether the window is currently maximized. When true, the middle button shows the restore icon (two overlapping squares); otherwise the maximize icon (single square).
    pub fn maximized(mut self, value: bool) -> Self {
        self.is_maximized = value;
        self
    }

    /// Sets horizontal spacing between the three window-control icons (minimize, maximize, close).
    pub fn icon_spacing(mut self, spacing: f32) -> Self {
        self.icon_spacing = spacing.clamp(0.0, 64.0);
        self
    }

    /// Sets which side of the titlebar the window controls appear on.
    pub fn controls_side(mut self, side: ControlsSide) -> Self {
        self.controls_side = side;
        self
    }
}

impl<'a, Message, Theme> From<TitleBarWindows<'a, Message, Theme>>
    for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
{
    fn from(value: TitleBarWindows<'a, Message, Theme>) -> Self {
        let to_message = value.on_message.expect(
            "titlebar_windows: on_message must be set before converting to Element (e.g. titlebar_windows(text(\"App\")).on_message(Message::Titlebar).into())",
        );
        build_titlebar_windows_element(
            value.title,
            value.style,
            value.height,
            value.is_maximized,
            value.icon_spacing,
            value.controls_side,
            to_message,
        )
    }
}

impl<'a, Message, Theme> TitleBarWindows<'a, Message, Theme>
where
    Message: Clone + 'a + 'static,
    Theme:
        button::Catalog + container::Catalog + svg::Catalog + iced::widget::text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
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
        surround_with_resize_edges(bar, content.into(), resize_edge_size, chrome, to_resize)
    }
}

/// Builds a custom titlebar element. Used by [From] and [titlebar_windows_with_style].
fn build_titlebar_windows_element<'a, Message, Theme>(
    title: Element<'a, Message, Theme, iced::Renderer>,
    style: style::TitlebarStyle,
    height: f32,
    is_maximized: bool,
    icon_spacing: f32,
    controls_side: ControlsSide,
    to_message: Box<dyn Fn(TitlebarMessage) -> Message + 'a>,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
{
    let draggable = draggable_title_area(title, &*to_message);

    let s_min = style;
    let s_max = style;
    let s_close = style;

    let min_icon = container(svg(minimize_handle()).width(10).height(10).style(
        move |_theme, _status| svg::Style {
            color: Some(s_min.icon),
        },
    ))
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

    let close_icon = container(svg(close_handle()).width(10).height(10).style(
        move |_theme, _status| svg::Style {
            color: Some(s_close.icon),
        },
    ))
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

    let row = if controls_side == ControlsSide::Left {
        row![controls, draggable]
    } else {
        row![draggable, controls]
    }
    .spacing(0)
    .height(height)
    .align_y(Alignment::Center);

    let bg = style.background;
    container(row)
        .height(height)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: bg.map(iced::Background::Color),
            ..Default::default()
        })
        .into()
}

/// Builds a custom titlebar with the given style (convenience wrapper around the builder).
///
/// Prefer the builder form: `titlebar_windows(title).style(style).maximized(is_maximized).on_message(to_message).into()`.
pub fn titlebar_windows_with_style<'a, Message, Theme>(
    title: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
    style: style::TitlebarStyle,
    is_maximized: bool,
    icon_spacing: f32,
    controls_side: ControlsSide,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
{
    build_titlebar_windows_element(
        title.into(),
        style,
        DEFAULT_TITLEBAR_HEIGHT,
        is_maximized,
        icon_spacing,
        controls_side,
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
