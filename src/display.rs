// salut-rs/src/display.rs
use crate::config::Config;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{size, Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn display_banner(banner: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?;
    let banner_lines: Vec<&str> = banner.lines().collect();
    let banner_height = banner_lines.len() as u16;
    let banner_width = banner_lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0) as u16;

    let start_row = if banner_height < rows {
        (rows - banner_height) / 2
    } else {
        0
    };
    let start_col = if banner_width < cols {
        (cols - banner_width) / 2
    } else {
        0
    };

    for (i, line) in banner_lines.iter().enumerate() {
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(start_col, start_row + i as u16),
            SetForegroundColor(Color::Green),
            Print(line),
            ResetColor
        )?;
    }
    Ok(())
}
pub fn display_shortcuts(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?; // Get terminal dimensions

    // Calculate the starting row for the shortcuts (e.g., 3/4 of the way down)
    let shortcuts_start_row = (rows as f32 * 0.75) as u16;

    // Build the entire shortcuts string first
    let mut shortcuts_string = String::new();
    for (key, shortcut) in &config.shortcuts {
        shortcuts_string.push_str(&format!(
            "({}) {} - {}  ",
            key, shortcut.icon, shortcut.name
        ));
    }

    // Calculate the starting column for centering
    let shortcuts_width = shortcuts_string.len() as u16;
    let start_col = if shortcuts_width < cols {
        (cols - shortcuts_width) / 2
    } else {
        0
    };

    execute!(
        stdout(),
        crossterm::cursor::MoveTo(start_col, shortcuts_start_row),
        SetForegroundColor(Color::Blue),
        Print(shortcuts_string),
        ResetColor
    )?;

    Ok(())
}
pub fn display_prompt() -> Result<(), Box<dyn std::error::Error>> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print("Enter command: "),
        ResetColor
    )?;
    stdout().flush()?;
    Ok(())
}
