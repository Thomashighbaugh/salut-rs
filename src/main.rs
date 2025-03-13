// salut-rs/src/main.rs
mod config;
mod display;

use config::get_config;
use display::{clear_screen, display_banner, display_prompt, display_shortcuts};
use std::io::{stdin, Read};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration.
    let config = get_config();

    // Initial clear screen and display.
    clear_screen()?;
    display_banner(&config)?;
    display_shortcuts(&config)?;
    display_prompt()?;

    // Input handling loop
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = input.trim();

    // Check if the input matches a shortcut and execute the command
    if let Some(shortcut) = config.shortcuts.get(input) {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&shortcut.command)
            .spawn()?;
        child.wait()?; // Wait for the command to finish
    } else if input == "q" {
        // Quit
        return Ok(());
    } else {
        println!("Invalid command: {}", input);
    }

    Ok(())
}
