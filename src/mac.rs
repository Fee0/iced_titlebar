//! macOS-style traffic lights titlebar (close, minimize, zoom) for iced with decorations disabled.
//!
//! Uses embedded SVG assets under `svg/macos/`. Built with [titlebar_mac] and
//! [TitleBarMac::on_message], emitting the same [TitlebarMessage](crate::common::TitlebarMessage) as the Windows-style bar.

use crate::common::{
    DEFAULT_TITLEBAR_HEIGHT, TitlebarMessage, draggable_title_area, surround_with_resize_edges,
};
use crate::style::{self, ControlsSide};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::svg;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Shell, Widget};
use iced::alignment::Horizontal;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{container, row};
use iced::{Alignment, Element, Event, Length, Rectangle, Size, mouse};

/// Diameter of each traffic light circle in logical pixels (SVG viewBox scales to this).
pub const TITLEBAR_MAC_LIGHT_DIAMETER: f32 = 18.0;

/// Horizontal gap between traffic light circles.
pub const TITLEBAR_MAC_LIGHT_SPACING: f32 = 8.0;

/// Default hit-target size when using [TitleBarMac::light_diameter] / [default_titlebar_mac_light_hit].
pub const TITLEBAR_MAC_LIGHT_HIT: f32 = TITLEBAR_MAC_LIGHT_DIAMETER * 2.0;

/// Suggested button hit size for a given icon diameter (about 2× the glyph).
#[must_use]
pub fn default_titlebar_mac_light_hit(light_diameter: f32) -> f32 {
    light_diameter * 2.0
}

// ── TrafficLightButton custom widget ──────────────────────────────────────────

#[derive(Default)]
struct TrafficLightState {
    hovered: bool,
    pressed: bool,
}

/// A single macOS traffic light button. Owns its three SVG states and manages
/// hover/press internally via iced's widget tree — no external state needed.
struct TrafficLightButton<Message> {
    normal: SvgHandle,
    hover: SvgHandle,
    press: SvgHandle,
    on_press: Message,
    /// Hit-target side length in logical pixels. The SVG is drawn centered within this.
    size: f32,
}

impl<Message, Theme> Widget<Message, Theme, iced::Renderer> for TrafficLightButton<Message>
where
    Message: Clone,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<TrafficLightState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(TrafficLightState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(
            Length::Fixed(self.size),
            Length::Fill,
            Size::new(self.size, self.size),
        );
        layout::Node::new(size)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::svg::Renderer as SvgRenderer;

        let state = tree.state.downcast_ref::<TrafficLightState>();
        let handle = if state.pressed {
            &self.press
        } else if state.hovered {
            &self.hover
        } else {
            &self.normal
        };

        let bounds = layout.bounds();
        let img_side = self.size.min(bounds.width).min(bounds.height);
        let svg_bounds = Rectangle {
            x: bounds.x + (bounds.width - img_side) / 2.0,
            y: bounds.y + (bounds.height - img_side) / 2.0,
            width: img_side,
            height: img_side,
        };

        renderer.draw_svg(
            svg::Svg {
                handle: handle.clone(),
                color: None,
                rotation: iced::Radians(0.0),
                opacity: 1.0,
            },
            svg_bounds,
            *viewport,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<TrafficLightState>();
        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let over = cursor.is_over(bounds);
                if over != state.hovered {
                    state.hovered = over;
                    if !over {
                        state.pressed = false;
                    }
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) if state.hovered || state.pressed => {
                state.hovered = false;
                state.pressed = false;
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if cursor.is_over(bounds) =>
            {
                state.pressed = true;
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if state.pressed => {
                let fired = cursor.is_over(bounds);
                state.pressed = false;
                shell.request_redraw();
                if fired {
                    shell.publish(self.on_press.clone());
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<TrafficLightState>();
        if state.hovered {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }
}

impl<'a, Message, Theme> From<TrafficLightButton<Message>>
    for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
{
    fn from(btn: TrafficLightButton<Message>) -> Self {
        Element::new(btn)
    }
}

// ── TitleBarMac builder ───────────────────────────────────────────────────────

/// macOS-style titlebar: traffic lights on the left, draggable title filling the rest.
///
/// Build with [titlebar_mac], chain options, then [.into()](Into::into) after [on_message](TitleBarMac::on_message).
pub struct TitleBarMac<'a, Message, Theme = iced::Theme> {
    /// Element shown in the draggable title area. Can be any iced widget (text, row, image, …).
    pub title: Element<'a, Message, Theme, iced::Renderer>,
    pub style: style::TitlebarStyle,
    pub height: f32,
    pub is_maximized: bool,
    /// Circle diameter for each traffic light SVG (logical pixels).
    pub light_diameter: f32,
    /// Horizontal spacing between the three traffic lights.
    pub icon_spacing: f32,
    /// Padding around the traffic lights group.
    pub lights_padding: iced::Padding,
    /// Which side of the bar the traffic lights appear on. Defaults to [ControlsSide::Left].
    pub controls_side: ControlsSide,
    pub resize_edge_size: Option<f32>,
    /// Optional visual border width (in pixels). When set, the drawn border is this thick while
    /// the drag zone remains `resize_edge_size`. Defaults to matching `resize_edge_size`.
    pub border_width: Option<f32>,
    pub on_message: Option<Box<dyn Fn(TitlebarMessage) -> Message + 'a>>,
    _theme: std::marker::PhantomData<Theme>,
}

impl<'a, Message, Theme> std::fmt::Debug for TitleBarMac<'a, Message, Theme> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TitleBarMac")
            .field("title", &"<Element>")
            .field("style", &self.style)
            .field("height", &self.height)
            .field("is_maximized", &self.is_maximized)
            .field("light_diameter", &self.light_diameter)
            .field("icon_spacing", &self.icon_spacing)
            .field("lights_padding", &self.lights_padding)
            .field("controls_side", &self.controls_side)
            .field("resize_edge_size", &self.resize_edge_size)
            .field("on_message", &self.on_message.is_some())
            .finish()
    }
}

/// Creates a [TitleBarMac] with defaults. Call [TitleBarMac::on_message] then [.into()](Into::into).
/// Pass any iced `Element` (or anything that converts `Into<Element>`) as the title.
pub fn titlebar_mac<'a, Message, Theme>(
    title: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
) -> TitleBarMac<'a, Message, Theme> {
    TitleBarMac {
        title: title.into(),
        style: style::TitlebarStyle::default(),
        height: 38.0,
        is_maximized: false,
        light_diameter: 8.0,
        icon_spacing: 6.0,
        lights_padding: iced::Padding::from([0.0, 12.0]),
        controls_side: ControlsSide::Left,
        resize_edge_size: None,
        border_width: None,
        on_message: None,
        _theme: std::marker::PhantomData,
    }
}

impl<'a, Message, Theme> TitleBarMac<'a, Message, Theme> {
    pub fn on_message(mut self, f: impl Fn(TitlebarMessage) -> Message + 'a) -> Self {
        self.on_message = Some(Box::new(f));
        self
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

    /// Sets the visual border width (in pixels) independently of the drag zone size.
    /// Useful for a thin 1px border with a larger drag hit zone.
    /// When not set, defaults to `resize_edge_size`.
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width.max(0.0));
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

    /// Sets padding around the traffic lights group (e.g. `Padding::default().left(8.0)`).
    pub fn lights_padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.lights_padding = padding.into();
        self
    }

    /// Sets which side of the titlebar the traffic lights appear on.
    pub fn controls_side(mut self, side: ControlsSide) -> Self {
        self.controls_side = side;
        self
    }
}

impl<'a, Message, Theme> From<TitleBarMac<'a, Message, Theme>>
    for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: container::Catalog + 'static,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(value: TitleBarMac<'a, Message, Theme>) -> Self {
        build_titlebar_mac_element(value)
    }
}

impl<'a, Message, Theme> TitleBarMac<'a, Message, Theme>
where
    Message: Clone + 'a + 'static,
    Theme: container::Catalog + iced::widget::text::Catalog + 'static,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    /// Titlebar on top of `content`, wrapped in resize handles (same behavior as [TitleBarWindows::with_content](crate::windows::TitleBarWindows::with_content)).
    pub fn with_content(
        self,
        content: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
        to_resize: impl Fn(iced::window::Direction) -> Message + 'a,
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let resize_edge_size = self.resize_edge_size;
        let border_width = self.border_width;
        let chrome = self.style;
        let bar: Element<'a, Message, Theme, iced::Renderer> = self.into();
        surround_with_resize_edges(
            bar,
            content.into(),
            resize_edge_size,
            border_width,
            chrome,
            to_resize,
        )
    }
}

/// Builds a custom titlebar with the given style (convenience wrapper around the builder).
///
/// Prefer the builder form: `titlebar_mac(title).style(style).maximized(is_maximized).light_diameter(d).on_message(to_message).into()`.
pub fn titlebar_mac_with_style<'a, Message, Theme>(
    title: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
    to_message: impl Fn(TitlebarMessage) -> Message + 'a,
    style: style::TitlebarStyle,
    is_maximized: bool,
    icon_spacing: f32,
    light_diameter: f32,
    controls_side: ControlsSide,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: container::Catalog + 'static,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    TitleBarMac {
        title: title.into(),
        style,
        height: DEFAULT_TITLEBAR_HEIGHT,
        is_maximized,
        light_diameter: light_diameter.clamp(4.0, 64.0),
        icon_spacing: icon_spacing.clamp(0.0, 64.0),
        lights_padding: iced::Padding::ZERO,
        controls_side,
        on_message: Some(Box::new(to_message)),
        resize_edge_size: None,
        border_width: None,
        _theme: std::marker::PhantomData,
    }
    .into()
}

fn build_titlebar_mac_element<'a, Message, Theme>(
    bar: TitleBarMac<'a, Message, Theme>,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'a + 'static,
    Theme: container::Catalog + 'static,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    let to_message = bar
        .on_message
        .expect("titlebar_mac: on_message must be set before converting to Element");
    let style = bar.style;
    let height = bar.height;
    let light_diameter = bar.light_diameter;
    let icon_spacing = bar.icon_spacing;
    let lights_padding = bar.lights_padding;
    let controls_side = bar.controls_side;

    let hit = default_titlebar_mac_light_hit(light_diameter);

    let close_msg = to_message(TitlebarMessage::Close);
    let min_msg = to_message(TitlebarMessage::Minimize);
    let max_msg = to_message(TitlebarMessage::ToggleMaximize);
    let draggable = draggable_title_area(bar.title, &*to_message);

    let close_btn: Element<'a, Message, Theme, iced::Renderer> = TrafficLightButton {
        normal: macos_close_normal(),
        hover: macos_close_hover(),
        press: macos_close_press(),
        on_press: close_msg,
        size: hit,
    }
    .into();

    let min_btn: Element<'a, Message, Theme, iced::Renderer> = TrafficLightButton {
        normal: macos_minimize_normal(),
        hover: macos_minimize_hover(),
        press: macos_minimize_press(),
        on_press: min_msg,
        size: hit,
    }
    .into();

    let max_btn: Element<'a, Message, Theme, iced::Renderer> = TrafficLightButton {
        normal: macos_maximize_normal(),
        hover: macos_maximize_hover(),
        press: macos_maximize_press(),
        on_press: max_msg,
        size: hit,
    }
    .into();

    let lights_row = row![close_btn, min_btn, max_btn]
        .spacing(icon_spacing)
        .align_y(Alignment::Center)
        .height(Length::Fill);

    let lights_block = container(lights_row)
        .padding(lights_padding)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Alignment::Center);

    let bar_row = if controls_side == ControlsSide::Left {
        row![lights_block, draggable]
    } else {
        row![draggable, lights_block]
    }
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

// ── Embedded SVG assets ───────────────────────────────────────────────────────

fn macos_close_normal() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/1-close-1-normal.svg"
        ))
        .to_vec(),
    )
}

fn macos_close_hover() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/2-close-2-hover.svg"
        ))
        .to_vec(),
    )
}

fn macos_close_press() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/2-close-3-press.svg"
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

fn macos_minimize_hover() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/2-minimize-2-hover.svg"
        ))
        .to_vec(),
    )
}

fn macos_minimize_press() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/2-minimize-3-press.svg"
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

fn macos_maximize_hover() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/3-maximize-2-hover.svg"
        ))
        .to_vec(),
    )
}

fn macos_maximize_press() -> SvgHandle {
    SvgHandle::from_memory(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/svg/macos/3-maximize-3-press.svg"
        ))
        .to_vec(),
    )
}
