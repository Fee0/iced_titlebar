//! Custom titlebar library for [iced](https://docs.rs/iced) applications.
//!
//! Use this when you disable window decorations (e.g. `Application::decorations(false)`) and want
//! to draw your own titlebar with drag, minimize, maximize, and close — using iced's built-in
//! [window](https://docs.rs/iced/latest/iced/window/) APIs under the hood.
//!
//! # Example
//!
//! ```ignore
//! use iced_custom_titlebar::{titlebar, TitlebarMessage};
//!
//! // In your view (builder style, like other iced widgets). Pass current maximized state so the middle button shows the correct icon:
//! let bar = titlebar("My App").maximized(is_maximized).on_message(Message::Titlebar).into();
//!
//! // In your update (with window_id from state, e.g. stored from window::open_events()):
//! Message::Titlebar(TitlebarMessage::StartDrag) => window::drag(window_id),
//! Message::Titlebar(TitlebarMessage::Minimize) => window::minimize(window_id, true),
//! Message::Titlebar(TitlebarMessage::ToggleMaximize) => window::toggle_maximize(window_id),
//! Message::Titlebar(TitlebarMessage::Close) => window::close(window_id),
//! ```
//!
//! # Traffic lights (macOS-style) titlebar
//!
//! ```ignore
//! use iced_custom_titlebar::{traffic_lights_titlebar, TitlebarMessage};
//!
//! let bar = traffic_lights_titlebar("My App")
//!     .maximized(is_maximized)
//!     .light_diameter(18.0)
//!     .on_message(Message::Titlebar)
//!     .into();
//! // Same TitlebarMessage mapping as above in `update`.
//! ```
//!
//! For resizing by dragging edges and corners, wrap your content with [resize_handles] and handle
//! the direction message with `window::drag_resize(window_id, direction)`.

pub mod common;
pub mod mac;
pub mod style;
pub mod windows;

pub use common::{
    DEFAULT_TITLEBAR_HEIGHT, RESIZE_CORNER_SIZE, RESIZE_EDGE_SIZE, TitlebarMessage, resize_handles,
};
pub use mac::{
    TRAFFIC_LIGHT_DIAMETER, TRAFFIC_LIGHT_HIT, TRAFFIC_LIGHT_SPACING, TRAFFIC_LIGHTS_LEFT_PADDING,
    TrafficLightsTitlebar, default_traffic_light_hit, traffic_lights_titlebar,
};
pub use style::{TitleAlignment, TitlebarStyle, TitlebarStylePreset};
pub use windows::{titlebar, titlebar_with_style, Titlebar, DEFAULT_ICON_WIDTH};
