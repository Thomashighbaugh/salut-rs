// salut-rs/src/config.rs
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub banner: String,
    pub shortcuts: HashMap<String, Shortcut>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shortcut {
    pub name: String,
    pub icon: String,
    pub command: String,
    pub description: Option<String>, // Optional description
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
            icon: " ".to_string(),
            command: "nvim".to_string(),
            description: Some("Open Neovim".to_string()),
        },
    );
    default_shortcuts.insert(
        "ft".to_string(),
        Shortcut {
            name: "Fastfetch".to_string(),
            icon: " ".to_string(),
            command: "fastfetch".to_string(),
            description: Some("Run Fastfetch".to_string()),
        },
    );
    default_shortcuts.insert(
        "zs".to_string(),
        Shortcut {
            name: "Zsh".to_string(),
            icon: "$ ".to_string(),
            command: "zsh".to_string(),
            description: Some("Start Zsh Shell".to_string()),
        },
    );

    default_shortcuts.insert(
        "bp".to_string(),
        Shortcut {
            name: "Btop".to_string(),
            icon: " ".to_string(),
            command: "btop".to_string(),
            description: Some("Start Btop".to_string()),
        },
    );

    let default_config = Config {
        banner: "Default Banner".to_string(), //A simpler banner as default
        shortcuts: default_shortcuts,
    };

    let toml_string = toml::to_string(&default_config)?;
    fs::create_dir_all(config_path.parent().unwrap())?; // Ensure directory exists.
    fs::write(config_path, toml_string)?;
    Ok(())
}

pub fn get_config() -> Config {
    match load_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading config: {}", err);
            // Fallback to an empty config or a hardcoded default.
            Config {
                banner: "Salut".to_string(),
                shortcuts: HashMap::new(),
            }
        }
    }
}
