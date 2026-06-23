//! Example: macOS-style traffic lights titlebar with decorations disabled.
//! Run with: cargo run --example mac
//!
//! The content area includes controls to change titlebar options at runtime (same idea as the `windows` example).

use iced::widget::{
    column, container as container_widget, pick_list, row, slider, text, text_input,
};
use iced::{Alignment, Element, Length, Padding, Settings, Subscription, Task};

use iced_custom_titlebar::{TitlebarMessage, TitlebarStyle, TitlebarStylePreset, titlebar_mac};

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .subscription(subscription)
        .decorations(false)
        .run()
}

struct State {
    window_id: Option<iced::window::Id>,
    title: String,
    height: f32,
    resize_edge: f32,
    style_preset: TitlebarStylePreset,
    is_maximized: bool,
    light_diameter: f32,
    icon_spacing: f32,
    lights_padding: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            window_id: None,
            title: "Traffic lights titlebar demo".to_string(),
            height: 38.0,
            resize_edge: 1.0,
            style_preset: TitlebarStylePreset::default(),
            is_maximized: false,
            light_diameter: 8.0,
            icon_spacing: 6.0,
            lights_padding: 12.0,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(iced::window::Id),
    Titlebar(TitlebarMessage),
    Resize(iced::window::Direction),
    TitleChanged(String),
    HeightChanged(f32),
    ResizeEdgeChanged(f32),
    StylePresetChanged(TitlebarStylePreset),
    LightDiameterChanged(f32),
    IconSpacingChanged(f32),
    LightsPaddingChanged(f32),
}

fn subscription(_state: &State) -> Subscription<Message> {
    iced::window::open_events().map(Message::WindowOpened)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::WindowOpened(id) => {
            state.window_id = Some(id);
            Task::none()
        }
        Message::Titlebar(tb) => {
            let Some(window_id) = state.window_id else {
                return Task::none();
            };
            match tb {
                TitlebarMessage::StartDrag => {
                    iced::window::drag::<()>(window_id).discard::<Message>()
                }
                TitlebarMessage::Minimize => {
                    iced::window::minimize::<()>(window_id, true).discard::<Message>()
                }
                TitlebarMessage::ToggleMaximize => {
                    state.is_maximized = !state.is_maximized;
                    iced::window::toggle_maximize::<()>(window_id).discard::<Message>()
                }
                TitlebarMessage::Close => iced::window::close::<()>(window_id).discard::<Message>(),
            }
        }
        Message::Resize(direction) => {
            let Some(window_id) = state.window_id else {
                return Task::none();
            };
            iced::window::drag_resize::<()>(window_id, direction).discard::<Message>()
        }
        Message::TitleChanged(s) => {
            state.title = s;
            Task::none()
        }
        Message::HeightChanged(h) => {
            state.height = h;
            Task::none()
        }
        Message::ResizeEdgeChanged(e) => {
            state.resize_edge = e;
            Task::none()
        }
        Message::StylePresetChanged(preset) => {
            state.style_preset = preset;
            Task::none()
        }
        Message::LightDiameterChanged(d) => {
            state.light_diameter = d;
            Task::none()
        }
        Message::IconSpacingChanged(s) => {
            state.icon_spacing = s;
            Task::none()
        }
        Message::LightsPaddingChanged(p) => {
            state.lights_padding = p;
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let title_label = text("Title:").size(14);
    let title_input = text_input("Window title", state.title.as_str())
        .on_input(Message::TitleChanged)
        .width(250);

    let height_label = text(format!("Height: {:.0} px", state.height)).size(14);
    let height_slider = slider(24.0..=48.0, state.height, Message::HeightChanged).width(200);

    let resize_label = text(format!("Resize edge: {:.1} px", state.resize_edge)).size(14);
    let resize_slider =
        slider(0.0..=10.0, state.resize_edge, Message::ResizeEdgeChanged).width(200);

    let light_label = text(format!(
        "Traffic light size: {:.0} px",
        state.light_diameter
    ))
    .size(14);
    let light_slider = slider(
        8.0..=32.0,
        state.light_diameter,
        Message::LightDiameterChanged,
    )
    .width(200);

    let icon_spacing_label = text(format!(
        "Icon spacing (traffic lights): {:.1} px",
        state.icon_spacing
    ))
    .size(14);
    let icon_spacing_slider =
        slider(0.0..=24.0, state.icon_spacing, Message::IconSpacingChanged).width(200);

    let lights_padding_label = text(format!(
        "Lights padding (h): {:.1} px",
        state.lights_padding
    ))
    .size(14_f32);
    let lights_padding_slider =
        slider(0.0..=32.0, state.lights_padding, Message::LightsPaddingChanged).width(200);

    let style_options = [TitlebarStylePreset::Dark, TitlebarStylePreset::Light];
    let style_pick = pick_list(
        style_options,
        Some(state.style_preset),
        Message::StylePresetChanged,
    )
    .width(120);

    let config_panel = column![
        text("Change options below; titlebar updates live.").size(14),
        row![title_label, title_input]
            .spacing(8)
            .align_y(Alignment::Center),
        row![height_label, height_slider]
            .spacing(8)
            .align_y(Alignment::Center),
        row![resize_label, resize_slider]
            .spacing(8)
            .align_y(Alignment::Center),
        row![light_label, light_slider]
            .spacing(8)
            .align_y(Alignment::Center),
        row![icon_spacing_label, icon_spacing_slider]
            .spacing(8)
            .align_y(Alignment::Center),
        row![lights_padding_label, lights_padding_slider]
            .spacing(8)
            .align_y(Alignment::Center),
        row![text("Style preset:").size(14), style_pick,]
            .spacing(8)
            .align_y(Alignment::Center),
    ]
    .spacing(12)
    .padding(Padding::from(20))
    .width(Length::Fill)
    .align_x(Alignment::Start);

    let style = TitlebarStyle::from(state.style_preset);

    let title_str = if state.title.is_empty() {
        "Traffic lights titlebar demo"
    } else {
        state.title.as_str()
    };

    // Pass any iced Element as the title — here a simple text widget.
    let title_element: Element<'_, Message> = container_widget(text(title_str).size(14))
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .into();

    let with_handles: Element<'_, Message> = titlebar_mac(title_element)
        .on_message(Message::Titlebar)
        .height(state.height)
        .resize_edge(state.resize_edge)
        .maximized(state.is_maximized)
        .light_diameter(state.light_diameter)
        .icon_spacing(state.icon_spacing)
        .lights_padding(Padding::from([0.0, state.lights_padding]))
        .style(style)
        .with_content(config_panel, Message::Resize);

    container_widget(with_handles)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
