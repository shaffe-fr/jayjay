// Release builds are a GUI app: opt out of the console subsystem so launching
// the executable does not spawn a terminal. Debug builds keep the console for
// logs.
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

fn main() {
    std::process::exit(jayjay_gpui::startup::run(
        &std::env::args().skip(1).collect::<Vec<_>>(),
    ));
}
