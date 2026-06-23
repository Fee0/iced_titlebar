//! macOS-style traffic lights titlebar (close, minimize, zoom) for iced with decorations disabled.
//!
//! Uses embedded SVG assets under `svg/macos/`. Built with [titlebar_mac] and
//! [TitleBarMac::on_message], emitting the same [TitlebarMessage](crate::common::TitlebarMessage) as the Windows-style bar.

use crate::common::{
    DEFAULT_TITLEBAR_HEIGHT, TitlebarMessage, draggable_title_area, surround_with_resize_edges,
};
use crate::style::{self, TitleAlignment};
use iced::alignment::Horizontal;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, container, row, svg, text};
use iced::{Alignment, Element, Length};

/// Diameter of each traffic light circle in logical pixels (SVG viewBox scales to this).
pub const TITLEBAR_MAC_LIGHT_DIAMETER: f32 = 18.0;

/// Horizontal gap between traffic light circles.
pub const TITLEBAR_MAC_LIGHT_SPACING: f32 = 8.0;

/// Left padding before the first traffic light.
pub const TITLEBAR_MAC_LIGHTS_LEFT_PADDING: f32 = 10.0;

/// Default hit-target size when using [TitleBarMac::light_diameter] / [default_titlebar_mac_light_hit].
pub const TITLEBAR_MAC_LIGHT_HIT: f32 = 36.0;

/// Suggested button hit size for a given icon diameter (about 2× the glyph, minimum 24px).
#[must_use]
pub fn default_titlebar_mac_light_hit(light_diameter: f32) -> f32 {
    (light_diameter * 2.0).max(24.0)
}

/// macOS-style titlebar: traffic lights on the left, draggable title filling the rest.
///
/// Build with [titlebar_mac], chain options, then [.into()](Into::into) after [on_message](TitleBarMac::on_message).
pub struct TitleBarMac<'a, Message, Theme = iced::Theme> {
    pub title: String,
    pub style: style::TitlebarStyle,
    pub height: f32,
    pub title_alignment: TitleAlignment,
    pub is_maximized: bool,
    /// Circle diameter for each traffic light SVG (logical pixels).
    pub light_diameter: f32,
    /// Horizontal spacing between the three traffic lights.
    pub icon_spacing: f32,
    pub resize_edge_size: Option<f32>,
    pub on_message: Option<Box<dyn Fn(TitlebarMessage) -> Message + 'a>>,
    _theme: std::marker::PhantomData<Theme>,
}

impl<'a, Message, Theme> std::fmt::Debug for TitleBarMac<'a, Message, Theme> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TitleBarMac")
            .field("title", &self.title)
            .field("style", &self.style)
            .field("height", &self.height)
            .field("title_alignment", &self.title_alignment)
            .field("is_maximized", &self.is_maximized)
            .field("light_diameter", &self.light_diameter)
            .field("icon_spacing", &self.icon_spacing)
            .field("resize_edge_size", &self.resize_edge_size)
            .field("on_message", &self.on_message.is_some())
            .finish()
    }
}

/// Creates a [TitleBarMac] with defaults. Call [TitleBarMac::on_message] then [.into()](Into::into).
pub fn titlebar_mac<Message, Theme>(title: impl ToString) -> TitleBarMac<'static, Message, Theme> {
    TitleBarMac {
        title: title.to_string(),
        style: style::TitlebarStyle::default(),
        height: DEFAULT_TITLEBAR_HEIGHT,
        title_alignment: TitleAlignment::default(),
        is_maximized: false,
        light_diameter: TITLEBAR_MAC_LIGHT_DIAMETER,
        icon_spacing: TITLEBAR_MAC_LIGHT_SPACING,
        resize_edge_size: None,
        on_message: None,
        _theme: std::marker::PhantomData,
    }
}

impl<'a, Message, Theme> TitleBarMac<'a, Message, Theme> {
    pub fn on_message<'b, F>(self, f: F) -> TitleBarMac<'b, Message, Theme>
    where
        F: Fn(TitlebarMessage) -> Message + 'b,
    {
        TitleBarMac {
            title: self.title,
            style: self.style,
            height: self.height,
            title_alignment: self.title_alignment,
            is_maximized: self.is_maximized,
            light_diameter: self.light_diameter,
            icon_spacing: self.icon_spacing,
            resize_edge_size: self.resize_edge_size,
            on_message: Some(Box::new(f)),
            _theme: std::marker::PhantomData,
        }
    }

    pub fn style(mut self, s: style::TitlebarStyle) -> Self {
        self.style = s;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn resize_edge(mut self, size: f32) -> Self {
        self.resize_edge_size = Some(size.max(0.0));
        self
    }

    pub fn title_alignment(mut self, a: TitleAlignment) -> Self {
        self.title_alignment = a;
        self
    }

    pub fn maximized(mut self, value: bool) -> Self {
        self.is_maximized = value;
        self
    }

    /// Sets the diameter of each traffic light icon in logical pixels (hit target scales with [default_titlebar_mac_light_hit]).
    pub fn light_diameter(mut self, diameter: f32) -> Self {
        self.light_diameter = diameter.clamp(4.0, 64.0);
        self
    }

    /// Sets horizontal spacing between the three traffic lights.
    pub fn icon_spacing(mut self, spacing: f32) -> Self {
        self.icon_spacing = spacing.clamp(0.0, 64.0);
        self
    }
}

impl<'a, Message, Theme> From<TitleBarMac<'a, Message, Theme>>
    for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    fn from(value: TitleBarMac<'a, Message, Theme>) -> Self {
        build_titlebar_mac_element(value)
    }
}

impl<'a, Message, Theme> TitleBarMac<'a, Message, Theme>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    /// Titlebar on top of `content`, wrapped in resize handles (same behavior as [TitleBarWindows::with_content](crate::windows::TitleBarWindows::with_content)).
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

/// Builds a custom titlebar with the given style (convenience wrapper around the builder).
///
/// Prefer the builder form: `titlebar_mac(title).style(style).maximized(is_maximized).light_diameter(d).on_message(to_message).into()`.
pub fn titlebar_mac_with_style<'a, Message, Theme>(
    title: impl ToString,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
    style: style::TitlebarStyle,
    title_alignment: TitleAlignment,
    is_maximized: bool,
    icon_spacing: f32,
    light_diameter: f32,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    TitleBarMac {
        title: title.to_string(),
        style,
        height: DEFAULT_TITLEBAR_HEIGHT,
        title_alignment,
        is_maximized,
        light_diameter: light_diameter.clamp(4.0, 64.0),
        icon_spacing: icon_spacing.clamp(0.0, 64.0),
        on_message: Some(Box::new(to_message)),
        resize_edge_size: None,
        _theme: std::marker::PhantomData,
    }
    .into()
}

fn build_titlebar_mac_element<'a, Message, Theme>(
    bar: TitleBarMac<'a, Message, Theme>,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: button::Catalog + container::Catalog + svg::Catalog + text::Catalog + 'static,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    let to_message = bar
        .on_message
        .expect("titlebar_mac: on_message must be set before converting to Element");
    let style = bar.style;
    let height = bar.height;
    let light_diameter = bar.light_diameter;
    let icon_spacing = bar.icon_spacing;

    let draggable = draggable_title_area(bar.title, style, bar.title_alignment, &*to_message);

    let d = light_diameter;
    let hit = default_titlebar_mac_light_hit(light_diameter);
    let s_close = style;
    let s_min = style;
    let s_max = style;

    let light_icon = |handle: SvgHandle| {
        container(svg(handle).width(d).height(d))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Alignment::Center)
    };

    let close_btn = button(light_icon(macos_close_normal()))
        .on_press(to_message(TitlebarMessage::Close))
        .style(move |theme, status| style::traffic_light_button_style(&s_close, theme, status))
        .width(Length::Fixed(hit))
        .height(Length::Fill);

    let min_btn = button(light_icon(macos_minimize_normal()))
        .on_press(to_message(TitlebarMessage::Minimize))
        .style(move |theme, status| style::traffic_light_button_style(&s_min, theme, status))
        .width(Length::Fixed(hit))
        .height(Length::Fill);

    let max_btn = button(light_icon(macos_maximize_normal()))
        .on_press(to_message(TitlebarMessage::ToggleMaximize))
        .style(move |theme, status| style::traffic_light_button_style(&s_max, theme, status))
        .width(Length::Fixed(hit))
        .height(Length::Fill);

    let lights_row = row![close_btn, min_btn, max_btn]
        .spacing(icon_spacing)
        .align_y(Alignment::Center)
        .height(Length::Fill);

    let lights_block = container(lights_row)
        .padding(iced::Padding::default().left(TITLEBAR_MAC_LIGHTS_LEFT_PADDING))
        .height(Length::Fill)
        .align_y(Alignment::Center);

    let bar_row = row![lights_block, draggable]
        .spacing(0)
        .align_y(Alignment::Center)
        .height(height);

    let bg = style.background;
    container(bar_row)
        .height(height)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: bg.map(iced::Background::Color),
            ..Default::default()
        })
        .into()
}

fn macos_close_normal() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/1-close-1-normal.svg"
        ))
        .to_vec(),
    )
}

fn macos_minimize_normal() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/2-minimize-1-normal.svg"
        ))
        .to_vec(),
    )
}

fn macos_maximize_normal() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/3-maximize-1-normal.svg"
        ))
        .to_vec(),
    )
}
