use std::process::Command;

use super::super::launcher::{EditorLaunch, detach_stdio, resolved_command};
use super::super::terminal::Terminal;

pub const EDITOR_OPTIONS: &[(&str, &str)] = &[
    ("system", "System Editor"),
    ("vscode", "Visual Studio Code"),
    ("vscodium", "VSCodium"),
    ("cursor", "Cursor"),
    ("zed", "Zed"),
    ("sublime", "Sublime Text"),
    ("vim", "Vim"),
    ("nvim", "Neovim"),
    ("custom", "Custom"),
];

pub const TERMINAL_OPTIONS: &[(&str, &str)] = &[
    ("terminal", "System Terminal"),
    ("powershell", "PowerShell"),
    ("cmd", "Command Prompt"),
    ("custom", "Custom"),
];

/// The System Editor on Windows opens the path with `explorer`, which honors the
/// default association for a file and opens the folder for a directory. `start`
/// raises an access-denied dialog on a directory, so it is not used here.
pub fn default_text_editor(path: &str) -> Option<EditorLaunch> {
    Some(EditorLaunch::from_argv(vec![
        "explorer".to_owned(),
        path.to_owned(),
    ]))
}

pub fn spawn_terminal(term: Terminal, cwd: &str, command: Option<&str>, custom: &str) -> bool {
    match term {
        Terminal::Custom if !custom.is_empty() => spawn_custom(custom, cwd, command),
        Terminal::PowerShell => spawn_shell("powershell", cwd, command),
        Terminal::Cmd => spawn_shell("cmd", cwd, command),
        _ => spawn_default(cwd, command),
    }
}

/// Windows Terminal opens at `cwd` via `-d`, then runs the shell as its start
/// command so PowerShell (not `wt`) receives `-NoExit`. Fall back to launching
/// the shell directly when Windows Terminal is not installed.
fn spawn_default(cwd: &str, command: Option<&str>) -> bool {
    if let Some((wt, _)) = resolved_command("wt") {
        let mut args = vec!["-d".to_owned(), cwd.to_owned()];
        // Only override the default profile when a command must run in it;
        // otherwise `wt -d <cwd>` opens the user's default profile.
        if command.is_some() {
            args.push("powershell".to_owned());
            args.extend(powershell_args(command));
        }
        if detach_stdio(Command::new(wt).args(args)).spawn().is_ok() {
            return true;
        }
    }
    spawn_shell("powershell", cwd, command) || spawn_shell("cmd", cwd, command)
}

/// A custom terminal is opened at `cwd` via `-d`, like Windows Terminal. Its
/// command-line syntax for running `_command` is unknown, so a terminal-editor
/// command is not forwarded here; it only opens the directory.
fn spawn_custom(custom: &str, cwd: &str, _command: Option<&str>) -> bool {
    let Some((binary, mut args)) = resolved_command(custom) else {
        return false;
    };
    args.push("-d".to_owned());
    args.push(cwd.to_owned());
    detach_stdio(Command::new(binary).args(args)).spawn().is_ok()
}

fn spawn_shell(shell: &str, cwd: &str, command: Option<&str>) -> bool {
    let Some((binary, _)) = resolved_command(shell) else {
        return false;
    };
    let args = if shell == "cmd" {
        cmd_args(command)
    } else {
        powershell_args(command)
    };
    detach_stdio(Command::new(binary).current_dir(cwd).args(args))
        .spawn()
        .is_ok()
}

/// `-NoExit` keeps the window open; `-Command` runs an optional command first.
fn powershell_args(command: Option<&str>) -> Vec<String> {
    match command {
        Some(command) => vec!["-NoExit".to_owned(), "-Command".to_owned(), command.to_owned()],
        None => vec!["-NoExit".to_owned()],
    }
}

/// `/k` keeps the window open after running an optional command.
fn cmd_args(command: Option<&str>) -> Vec<String> {
    match command {
        Some(command) => vec!["/k".to_owned(), command.to_owned()],
        None => Vec::new(),
    }
}
