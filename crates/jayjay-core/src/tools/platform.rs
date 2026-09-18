#[cfg(target_os = "macos")]
mod macos;

#[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
pub use linux::{EDITOR_OPTIONS, TERMINAL_OPTIONS, default_text_editor, spawn_terminal};
#[cfg(target_os = "windows")]
pub use windows::{EDITOR_OPTIONS, TERMINAL_OPTIONS, default_text_editor, spawn_terminal};
#[cfg(target_os = "macos")]
pub use macos::{EDITOR_OPTIONS, TERMINAL_OPTIONS, spawn_terminal};
