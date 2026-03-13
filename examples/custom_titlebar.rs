//! Example: custom titlebar with decorations disabled.
//! Run with: cargo run --example custom_titlebar

use iced::widget::container;
use iced::widget::{container as container_widget, text};
use iced::{Color, Element, Length, Subscription, Task};

use iced_custom_titlebar::{titlebar, TitlebarMessage};

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .subscription(subscription)
        .decorations(false)
        .run()
}

#[derive(Default)]
struct State {
    window_id: Option<iced::window::Id>,
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(iced::window::Id),
    Titlebar(TitlebarMessage),
    Resize(iced::window::Direction),
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
    }
}

fn view(_state: &State) -> Element<'_, Message> {
    let content = container_widget(
        text("Custom titlebar — drag the bar, use the buttons. Resize from edges and corners.")
            .size(16),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    let with_handles: Element<'_, Message> = titlebar("Custom Titlebar Demo")
        .on_message(Message::Titlebar)
        .resize_edge(1.0)
        .with_content(content, Message::Resize);

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
