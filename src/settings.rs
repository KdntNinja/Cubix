use bevy::prelude::*;
use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct PlayerSettings {
    pub speed: f32,
    pub sensitivity: f32,
    pub jump_force: f32,
    pub gravity: f32,
}

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct WorldSettings {
    pub block_size: f32,
    pub chunk_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct Settings {
    pub player: PlayerSettings,
    pub world: WorldSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            player: PlayerSettings {
                speed: 5.0,
                sensitivity: 0.1,
                jump_force: 8.0,
                gravity: 20.0,
            },
            world: WorldSettings {
                block_size: 1.0,
                chunk_size: 10,
            },
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Path::new("settings.toml");

        // If the file doesn't exist, create it with default settings
        if !config_path.exists() {
            let default_settings = Settings::default();
            let toml = toml::to_string_pretty(&default_settings)
                .expect("Failed to serialize default settings");

            fs::write(config_path, toml).expect("Failed to write default settings file");

            return Ok(default_settings);
        }

        // Load settings from the file
        let s = Config::builder()
            .add_source(File::from(config_path))
            .build()?;

        s.try_deserialize()
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        match Settings::load() {
            Ok(settings) => {
                info!("Settings loaded successfully");
                app.insert_resource(settings);
            }
            Err(e) => {
                error!("Failed to load settings: {}", e);
                app.insert_resource(Settings::default());
            }
        }
    }
}
