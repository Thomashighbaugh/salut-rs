// salut-rs/src/display.rs
use crate::config::{generate_banner, parse_color, Config};
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

pub fn display_banner(config: &Config, banner: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?;
    let banner_lines: Vec<&str> = banner.lines().collect();
    let banner_width = banner_lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0) as u16;

    let start_row = rows / 8; // One-quarter down from the top.

    let start_col = if banner_width < cols {
        (cols - banner_width) / 2
    } else {
        0
    };

    let banner_color = match &config.banner_color {
        Some(color_str) => parse_color(color_str)?,
        None => Color::Green, // Default color
    };
    execute!(stdout(), SetForegroundColor(banner_color))?;

    for (i, line) in banner_lines.iter().enumerate() {
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(start_col, start_row + i as u16),
            Print(line),
        )?;
    }
    execute!(stdout(), ResetColor)?; // Reset color
    Ok(())
}

pub fn display_shortcuts(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?; // Get terminal dimensions

    // Calculate the starting row for the shortcuts.
    let banner_height = generate_banner(config)?.lines().count() as u16;
    let shortcuts_start_row = rows / 2 + banner_height / 2 + 3;

    // Get and set the shortcuts color.
    let shortcuts_color = match &config.shortcuts_color {
        Some(color_str) => parse_color(color_str)?,
        None => Color::Blue, // Default
    };
    execute!(stdout(), SetForegroundColor(shortcuts_color))?;

    // Build the entire shortcuts string first, with better spacing.
    let mut shortcuts_string = String::new();
    for (key, shortcut) in &config.shortcuts {
        //the icon
        if let Some(icon) = &shortcut.icon {
            shortcuts_string.push_str(&format!("{} ", icon)); // Add space after icon
        }
        //The name of the shortcut
        shortcuts_string.push_str(&format!("{:<4} ", shortcut.name)); // Left-align names, fixed width

        //The key
        shortcuts_string.push_str(&format!("({})     ", key));
        // Add spacing between entries
        shortcuts_string.push(' '); // Keep a space between entries
    }

    // Calculate the starting column for centering.
    let shortcuts_width = console::measure_text_width(&shortcuts_string) as u16;

    let start_col = if shortcuts_width < cols {
        (cols - shortcuts_width) / 2
    } else {
        0
    };

    execute!(
        stdout(),
        crossterm::cursor::MoveTo(start_col, shortcuts_start_row),
        Print(shortcuts_string),
    )?;
    execute!(stdout(), ResetColor)?;

    Ok(())
}
pub fn display_prompt(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?;
    let prompt_text = "Enter command: ";
    let prompt_width = console::measure_text_width(prompt_text) as u16;

    // Calculate centered column
    let start_col = if prompt_width < cols {
        (cols - prompt_width) / 2
    } else {
        0
    };
    let prompt_start_row = rows - 1;

    let prompt_color = match &config.prompt_color {
        Some(color_str) => parse_color(color_str)?,
        None => Color::Yellow, // Default
    };

    execute!(
        stdout(),
        crossterm::cursor::MoveTo(start_col, prompt_start_row)
    )?; //place it at bottom
    execute!(
        stdout(),
        SetForegroundColor(prompt_color),
        Print(prompt_text),
        ResetColor
    )?;
    stdout().flush()?;
    Ok(())
}
