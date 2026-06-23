# iced_title_bar

Custom window titlebar for [iced](https://iced.rs): drag region, minimize, maximize/restore, and close. Messages map
to [`iced::window`](https://docs.rs/iced/latest/iced/window/) tasks in your `update` handler.

**Dependency:** `iced` 0.14 with the `svg` feature.

## Usage

- **Windows-style** (controls on the right): `titlebar_windows` → `TitleBarWindows`
- **macOS-style** (traffic lights on the left): `titlebar_mac` → `TitleBarMac`

Both emit `TitlebarMessage` (`StartDrag`, `Minimize`, `ToggleMaximize`, `Close`). Use `resize_handles` (or
`TitleBarWindows::with_content` / `TitleBarMac::with_content`) for edge/corner resize.

Optional one-shot builders: `titlebar_windows_with_style`, `titlebar_mac_with_style`.

## Examples

```bash
cargo run --example windows
cargo run --example mac
```
