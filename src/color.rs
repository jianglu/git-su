// Color output: only when stdout is TTY and git color.ui allows

use colored::Colorize;
use std::io::IsTerminal;

fn color_enabled() -> bool {
    std::io::stdout().is_terminal()
        && crate::git::Git::color_output()
}

pub fn label(s: &str) -> String {
    if color_enabled() {
        s.cyan().bold().to_string()
    } else {
        s.to_string()
    }
}

pub fn success(s: &str) -> String {
    if color_enabled() {
        s.green().to_string()
    } else {
        s.to_string()
    }
}

pub fn error(s: &str) -> String {
    if color_enabled() {
        s.red().to_string()
    } else {
        s.to_string()
    }
}

pub fn dim(s: &str) -> String {
    if color_enabled() {
        s.dimmed().to_string()
    } else {
        s.to_string()
    }
}
