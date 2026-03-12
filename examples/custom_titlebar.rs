//! Example: custom titlebar with decorations disabled.
//! Run with: cargo run --example custom_titlebar

use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length, Subscription, Task};

use iced_custom_titlebar::{resize_handles, titlebar, TitlebarMessage};

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
    let bar = titlebar("Custom Titlebar Demo", Message::Titlebar);
    let content = container(
        text("Custom titlebar — drag the bar, use the buttons. Resize from edges and corners.")
            .size(16),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    let inner = column![bar, content]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center);

    resize_handles(inner, Message::Resize).into()
}
