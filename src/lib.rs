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
//! // In your view:
//! let bar = titlebar("My App", Message::Titlebar);
//!
//! // In your update (with window_id from state, e.g. stored from window::open_events()):
//! Message::Titlebar(TitlebarMessage::StartDrag) => window::drag(window_id),
//! Message::Titlebar(TitlebarMessage::Minimize) => window::minimize(window_id, true),
//! Message::Titlebar(TitlebarMessage::ToggleMaximize) => window::toggle_maximize(window_id),
//! Message::Titlebar(TitlebarMessage::Close) => window::close(window_id),
//! ```
//!
//! For resizing by dragging edges and corners, wrap your content with [resize_handles] and handle
//! the direction message with `window::drag_resize(window_id, direction)`.

pub mod titlebar;
pub mod resize;

pub use titlebar::{titlebar, TitlebarMessage, DEFAULT_TITLEBAR_HEIGHT};
pub use resize::{resize_handles, RESIZE_EDGE_SIZE, RESIZE_CORNER_SIZE};
