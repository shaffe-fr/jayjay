//! Client-side-decoration caption controls for Windows and Linux, where the OS
//! draws no titlebar buttons. Each button carries only its `WindowControlArea`
//! so the OS caption hit-test drives it (maximize/restore, snap-layouts); a
//! click handler would fire alongside the OS action and cancel restore.
#![cfg(not(target_os = "macos"))]

use gpui::{
    AnyElement, InteractiveElement, IntoElement, ParentElement, Styled, WindowControlArea, div, px,
    rgb,
};

use crate::app::theme::Theme;
use crate::ui::icons::{self, glyph};

/// Minimize, maximize/restore, and close, laid out left to right. `is_maximized`
/// selects the restore vs maximize glyph. Height fills the parent titlebar.
pub(crate) fn window_controls(is_maximized: bool, t: &Theme) -> AnyElement {
    let maximize_glyph = if is_maximized {
        glyph::COPY
    } else {
        glyph::SQUARE
    };
    div()
        .flex()
        .flex_row()
        .items_center()
        .h_full()
        .child(caption_button(
            "window-minimize",
            glyph::MINUS,
            WindowControlArea::Min,
            false,
            t,
        ))
        .child(caption_button(
            "window-maximize",
            maximize_glyph,
            WindowControlArea::Max,
            false,
            t,
        ))
        .child(caption_button(
            "window-close",
            glyph::X,
            WindowControlArea::Close,
            true,
            t,
        ))
        .into_any_element()
}

fn caption_button(
    id: &'static str,
    glyph_str: &'static str,
    area: WindowControlArea,
    danger: bool,
    t: &Theme,
) -> AnyElement {
    let hover_bg = if danger { t.error_fg } else { t.row_alt_bg };
    div()
        .id(id)
        .debug_selector(move || id.to_owned())
        .flex()
        .items_center()
        .justify_center()
        .w(px(38.))
        .h_full()
        .cursor_pointer()
        .window_control_area(area)
        .hover(|style| style.bg(rgb(hover_bg)))
        .child(icons::icon(glyph_str, 11., t.fg_dim))
        .into_any_element()
}
