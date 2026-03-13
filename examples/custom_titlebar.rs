//! Example: custom titlebar with decorations disabled.
//! Run with: cargo run --example custom_titlebar
//!
//! The content area includes controls to change all titlebar options at runtime.

use iced::widget::container;
use iced::widget::{column, container as container_widget, pick_list, row, slider, text, text_input};
use iced::{Alignment, Color, Element, Length, Padding, Subscription, Task};

use iced_custom_titlebar::{titlebar, TitlebarMessage, TitlebarStyle, TitleAlignment};

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .subscription(subscription)
        .decorations(false)
        .run()
}

struct State {
    window_id: Option<iced::window::Id>,
    title: String,
    height: f32,
    resize_edge: f32,
    title_alignment: TitleAlignment,
    style_preset: StylePreset,
}

impl Default for State {
    fn default() -> Self {
        Self {
            window_id: None,
            title: "Custom Titlebar Demo".to_string(),
            height: 32.0,
            resize_edge: 1.0,
            title_alignment: TitleAlignment::default(),
            style_preset: StylePreset::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum StylePreset {
    #[default]
    Dark,
    Light,
}

impl std::fmt::Display for StylePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StylePreset::Dark => write!(f, "Dark"),
            StylePreset::Light => write!(f, "Light"),
        }
    }
}

fn titlebar_style_for(preset: StylePreset) -> TitlebarStyle {
    match preset {
        StylePreset::Dark => TitlebarStyle {
            bar: Color::from_rgb8(0, 0, 0),
            button_hover: Color::from_rgb8(60, 60, 60),
            close_hover: Color::from_rgb8(232, 17, 35),
            icon: Color::from_rgb8(255, 255, 255),
        },
        StylePreset::Light => TitlebarStyle {
            bar: Color::from_rgb8(240, 240, 240),
            button_hover: Color::from_rgb8(220, 220, 220),
            close_hover: Color::from_rgb8(232, 17, 35),
            icon: Color::from_rgb8(40, 40, 40),
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
    TitleAlignmentChanged(TitleAlignment),
    StylePresetChanged(StylePreset),
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
                TitlebarMessage::StartDrag => iced::window::drag::<()>(window_id).discard::<Message>(),
                TitlebarMessage::Minimize => iced::window::minimize::<()>(window_id, true).discard::<Message>(),
                TitlebarMessage::ToggleMaximize => iced::window::toggle_maximize::<()>(window_id).discard::<Message>(),
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
        Message::TitleAlignmentChanged(a) => {
            state.title_alignment = a;
            Task::none()
        }
        Message::StylePresetChanged(p) => {
            state.style_preset = p;
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
    let height_slider = slider(24.0..=48.0, state.height, Message::HeightChanged)
        .width(200);

    let resize_label = text(format!("Resize edge: {:.1} px", state.resize_edge)).size(14);
    let resize_slider = slider(0.0..=10.0, state.resize_edge, Message::ResizeEdgeChanged)
        .width(200);

    let alignment_options = [TitleAlignment::Left, TitleAlignment::Center, TitleAlignment::Right];
    let alignment_pick = pick_list(
        alignment_options,
        Some(state.title_alignment),
        Message::TitleAlignmentChanged,
    )
    .width(120);

    let style_options = [StylePreset::Dark, StylePreset::Light];
    let style_pick = pick_list(style_options, Some(state.style_preset), Message::StylePresetChanged)
        .width(120);

    let config_panel = column![
        text("Change options below; titlebar updates live.").size(14),
        row![title_label, title_input].spacing(8).align_y(Alignment::Center),
        row![height_label, height_slider].spacing(8).align_y(Alignment::Center),
        row![resize_label, resize_slider].spacing(8).align_y(Alignment::Center),
        row![
            text("Title alignment:").size(14),
            alignment_pick,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        row![
            text("Style preset:").size(14),
            style_pick,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    ]
    .spacing(12)
    .padding(Padding::from(20))
    .width(Length::Fill)
    .align_x(Alignment::Start);

    let style = titlebar_style_for(state.style_preset);

    let title_str = if state.title.is_empty() {
        "Custom Titlebar Demo"
    } else {
        state.title.as_str()
    };

    let with_handles: Element<'_, Message> = titlebar(title_str)
        .on_message(Message::Titlebar)
        .height(state.height)
        .resize_edge(state.resize_edge)
        .title_alignment(state.title_alignment)
        .style(style)
        .with_content(config_panel, Message::Resize);

    container_widget(with_handles)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| {
            container::Style::default().border(
                iced::Border::default()
                    .width(1.0)
                    .color(Color::from_rgb8(160, 160, 160)),
            )
        })
        .into()
}
