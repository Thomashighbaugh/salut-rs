// salut-rs/src/display.rs
use crate::config::Config;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn display_banner(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print(format!("{}\n", config.banner)),
        ResetColor
    )?;
    Ok(())
}

pub fn display_shortcuts(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    for (key, shortcut) in &config.shortcuts {
        execute!(
            stdout(),
            SetForegroundColor(Color::Blue),
            Print(format!("({})", key)),
            ResetColor,
            Print(format!(" {} - {}\n", shortcut.icon, shortcut.name))
        )?;
    }
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
