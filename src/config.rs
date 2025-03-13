// salut-rs/src/config.rs
use crossterm::style::Color;
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
 // Import FromStr
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub banner: String,
    pub figlet_font: Option<String>,     // Optional figlet font
    pub banner_color: Option<String>,    // String representation of color
    pub shortcuts_color: Option<String>, // String representation
    pub prompt_color: Option<String>,    // String representation
    pub shortcuts: HashMap<String, Shortcut>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shortcut {
    pub name: String,
    pub icon: Option<String>, // Icon is now optional
    pub command: String,
    pub description: Option<String>, // Optional description
}

// Helper function to convert color string to crossterm::style::Color.
pub fn parse_color(color_str: &str) -> Result<Color, Box<dyn Error>> {
    match color_str.to_lowercase().as_str() {
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),
        "darkgrey" | "darkgray" => Ok(Color::DarkGrey), // Support both spellings
        "darkred" => Ok(Color::DarkRed),
        "darkgreen" => Ok(Color::DarkGreen),
        "darkyellow" => Ok(Color::DarkYellow),
        "darkblue" => Ok(Color::DarkBlue),
        "darkmagenta" => Ok(Color::DarkMagenta),
        "darkcyan" => Ok(Color::DarkCyan),
        "darkwhite" => Ok(Color::DarkGrey), // Use DarkGrey for consistency
        _ => Err(format!("Invalid color: {}", color_str).into()), // Return error for unknown colors
    }
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_path = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("salut-rs/config.toml");

    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    let contents = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

fn create_default_config(config_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut default_shortcuts = HashMap::new();
    default_shortcuts.insert(
        "nv".to_string(),
        Shortcut {
            name: "Neovim".to_string(),
            icon: Some(" ".to_string()),
            command: "nvim".to_string(),
            description: Some("Open Neovim".to_string()),
        },
    );
    default_shortcuts.insert(
        "ft".to_string(),
        Shortcut {
            name: "Fastfetch".to_string(),
            icon: Some(" ".to_string()),
            command: "fastfetch".to_string(),
            description: Some("Run Fastfetch".to_string()),
        },
    );
    default_shortcuts.insert(
        "zs".to_string(),
        Shortcut {
            name: "Zsh".to_string(),
            icon: Some("$ ".to_string()),
            command: "zsh".to_string(),
            description: Some("Start Zsh Shell".to_string()),
        },
    );

    default_shortcuts.insert(
        "bp".to_string(),
        Shortcut {
            name: "Btop".to_string(),
            icon: Some(" ".to_string()),
            command: "btop".to_string(),
            description: Some("Start Btop".to_string()),
        },
    );
    let default_config = Config {
        banner: "Default Banner".to_string(),
        figlet_font: Some("chunky".to_string()),
        banner_color: Some("Green".to_string()), // Default: Green
        shortcuts_color: Some("Blue".to_string()), // Default: Blue
        prompt_color: Some("Yellow".to_string()), // Default: Yellow
        shortcuts: default_shortcuts,
    };

    let toml_string = toml::to_string(&default_config)?;
    fs::create_dir_all(config_path.parent().unwrap())?; // Ensure directory exists.
    fs::write(config_path, toml_string)?;
    Ok(())
}

// Modified get_config to handle color parsing.
pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config = load_config()?;
    Ok(config)
}

// Helper function to generate the banner using figlet.
pub fn generate_banner(config: &Config) -> Result<String, Box<dyn Error>> {
    let font = config.figlet_font.as_deref().unwrap_or("chunky");
    let output = Command::new("figlet")
        .arg("-f")
        .arg(font)
        .arg(&config.banner)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(format!("Figlet failed: {}", String::from_utf8(output.stderr)?).into())
    }
}
