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

pub fn display_banner(config: &Config, banner: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let color_code = config.banner_color.as_deref().unwrap_or("\x1b[32m"); // Default to green
    execute!(stdout(), Print(color_code))?;

    for (i, line) in banner_lines.iter().enumerate() {
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(start_col, start_row + i as u16),
            //SetForegroundColor(Color::Green),
            Print(line),
            //ResetColor
        )?;
    }
    execute!(stdout(), Print("\x1b[0m"))?; // Reset color
    Ok(())
}
pub fn display_shortcuts(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?; // Get terminal dimensions

    // Calculate the starting row for the shortcuts (e.g., 3/4 of the way down)
    let shortcuts_start_row = (rows as f32 * 0.75) as u16;

    // Build the entire shortcuts string first
    let mut shortcuts_string = String::new();
    for (key, shortcut) in &config.shortcuts {
        // Use the configured color, or default to blue if not set
        let color_code = config.shortcuts_color.as_deref().unwrap_or("\x1b[34m");
        shortcuts_string.push_str(color_code);

        shortcuts_string.push_str(&format!("({})", key));
        shortcuts_string.push_str("\x1b[0m"); // Reset color

        if let Some(icon) = &shortcut.icon {
            shortcuts_string.push_str(&format!(" {} ", icon));
        }
        shortcuts_string.push_str(&format!("{}  ", shortcut.name));
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
        //SetForegroundColor(Color::Blue),
        Print(shortcuts_string),
        //ResetColor
    )?;

    Ok(())
}
pub fn display_prompt(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, _rows) = size()?;
    let prompt_text = "Enter command: ";
    let prompt_width = prompt_text.len() as u16;
    let color_code = config.prompt_color.as_deref().unwrap_or("\x1b[33m"); // Default yellow

    // Calculate centered column
    let start_col = if prompt_width < cols {
        (cols - prompt_width) / 2
    } else {
        0
    };
    execute!(stdout(), crossterm::cursor::MoveTo(start_col, _rows - 1))?; //place it at bottom
    execute!(
        stdout(),
        //SetForegroundColor(Color::Yellow),
        Print(color_code),
        Print(prompt_text),
        Print("\x1b[0m")
    )?;
    stdout().flush()?;
    Ok(())
}
