//! macOS-style traffic lights titlebar (close, minimize, zoom) for iced with decorations disabled.
//!
//! Uses embedded SVG assets under `svg/macos/`. Built with [traffic_lights_titlebar] and
//! [TrafficLightsTitlebar::on_message], emitting the same [TitlebarMessage](crate::common::TitlebarMessage) as the Windows-style bar.

use crate::common::{
    draggable_title_area, surround_with_resize_edges, TitlebarMessage, DEFAULT_TITLEBAR_HEIGHT,
};
use crate::style::{self, TitleAlignment};
use iced::alignment::Horizontal;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, container, row, svg};
use iced::{Alignment, Element, Length};

/// Diameter of each traffic light circle in logical pixels (SVG viewBox scales to this).
pub const TRAFFIC_LIGHT_DIAMETER: f32 = 18.0;

/// Horizontal gap between traffic light circles.
pub const TRAFFIC_LIGHT_SPACING: f32 = 8.0;

/// Left padding before the first traffic light.
pub const TRAFFIC_LIGHTS_LEFT_PADDING: f32 = 10.0;

/// Default hit-target size when using [TrafficLightsTitlebar::light_diameter] / [default_traffic_light_hit].
pub const TRAFFIC_LIGHT_HIT: f32 = 36.0;

/// Suggested button hit size for a given icon diameter (about 2× the glyph, minimum 24px).
#[must_use]
pub fn default_traffic_light_hit(light_diameter: f32) -> f32 {
    (light_diameter * 2.0).max(24.0)
}

/// macOS-style titlebar: traffic lights on the left, draggable title filling the rest.
///
/// Build with [traffic_lights_titlebar], chain options, then [.into()](Into::into) after [on_message](TrafficLightsTitlebar::on_message).
pub struct TrafficLightsTitlebar<'a, Message> {
    pub title: String,
    pub style: style::TitlebarStyle,
    pub height: f32,
    pub title_alignment: TitleAlignment,
    pub is_maximized: bool,
    /// Circle diameter for each traffic light SVG (logical pixels).
    pub light_diameter: f32,
    pub resize_edge_size: Option<f32>,
    pub on_message: Option<Box<dyn Fn(TitlebarMessage) -> Message + 'a>>,
}

impl<'a, Message> std::fmt::Debug for TrafficLightsTitlebar<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrafficLightsTitlebar")
            .field("title", &self.title)
            .field("style", &self.style)
            .field("height", &self.height)
            .field("title_alignment", &self.title_alignment)
            .field("is_maximized", &self.is_maximized)
            .field("light_diameter", &self.light_diameter)
            .field("resize_edge_size", &self.resize_edge_size)
            .field("on_message", &self.on_message.is_some())
            .finish()
    }
}

/// Creates a [TrafficLightsTitlebar] with defaults. Call [TrafficLightsTitlebar::on_message] then [.into()](Into::into).
pub fn traffic_lights_titlebar<Message>(title: impl ToString) -> TrafficLightsTitlebar<'static, Message> {
    TrafficLightsTitlebar {
        title: title.to_string(),
        style: style::TitlebarStyle::default(),
        height: DEFAULT_TITLEBAR_HEIGHT,
        title_alignment: TitleAlignment::default(),
        is_maximized: false,
        light_diameter: TRAFFIC_LIGHT_DIAMETER,
        resize_edge_size: None,
        on_message: None,
    }
}

impl<'a, Message> TrafficLightsTitlebar<'a, Message> {
    pub fn on_message<'b, F>(self, f: F) -> TrafficLightsTitlebar<'b, Message>
    where
        F: Fn(TitlebarMessage) -> Message + 'b,
    {
        TrafficLightsTitlebar {
            title: self.title,
            style: self.style,
            height: self.height,
            title_alignment: self.title_alignment,
            is_maximized: self.is_maximized,
            light_diameter: self.light_diameter,
            resize_edge_size: self.resize_edge_size,
            on_message: Some(Box::new(f)),
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

    /// Sets the diameter of each traffic light icon in logical pixels (hit target scales with [default_traffic_light_hit]).
    pub fn light_diameter(mut self, diameter: f32) -> Self {
        self.light_diameter = diameter.clamp(4.0, 64.0);
        self
    }
}

impl<'a, Message> From<TrafficLightsTitlebar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    fn from(value: TrafficLightsTitlebar<'a, Message>) -> Self {
        let on_message = value.on_message.expect(
            "traffic_lights_titlebar: on_message must be set before converting to Element",
        );
        build_traffic_lights_titlebar_element(
            value.title,
            value.style,
            value.height,
            value.title_alignment,
            value.is_maximized,
            value.light_diameter,
            on_message,
        )
    }
}

impl<'a, Message> TrafficLightsTitlebar<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    /// Titlebar on top of `content`, wrapped in resize handles (same behavior as [crate::Titlebar::with_content](crate::Titlebar::with_content)).
    pub fn with_content(
        self,
        content: impl Into<Element<'a, Message>>,
        to_resize: impl Fn(iced::window::Direction) -> Message + 'a,
    ) -> Element<'a, Message> {
        let resize_edge_size = self.resize_edge_size;
        let chrome = self.style;
        let bar: Element<'a, Message> = self.into();
        surround_with_resize_edges(
            bar,
            content.into(),
            resize_edge_size,
            chrome,
            to_resize,
        )
    }
}

fn build_traffic_lights_titlebar_element<'a, Message>(
    title_str: String,
    style: style::TitlebarStyle,
    height: f32,
    title_alignment: TitleAlignment,
    #[allow(unused_variables)] is_maximized: bool,
    light_diameter: f32,
    to_message: Box<dyn Fn(TitlebarMessage) -> Message + 'a>,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let draggable = draggable_title_area(title_str, style, title_alignment, &*to_message);

    let d = light_diameter;
    let hit = default_traffic_light_hit(light_diameter);
    let s_close = style;
    let s_min = style;
    let s_max = style;
    let s_bar = style;

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
        .spacing(TRAFFIC_LIGHT_SPACING)
        .align_y(Alignment::Center)
        .height(Length::Fill);

    let lights_block = container(lights_row)
        .padding(iced::Padding::default().left(TRAFFIC_LIGHTS_LEFT_PADDING))
        .height(Length::Fill)
        .align_y(Alignment::Center);

    let bar_row = row![lights_block, draggable]
        .spacing(0)
        .align_y(Alignment::Center)
        .height(height);

    container(bar_row)
        .style(move |_theme| style::bar_container_style(&s_bar))
        .height(height)
        .width(Length::Fill)
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
