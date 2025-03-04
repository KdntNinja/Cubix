use serde::{Deserialize, Serialize};
use std::fs::{self, File};
pub use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Window settings
    pub window: WindowConfig,
    // Camera settings
    pub camera: CameraConfig,
    // Controls settings
    pub controls: ControlsConfig,
    // Physics settings
    pub physics: PhysicsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    pub sensitivity: f32,
    pub fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlsConfig {
    pub cursor_locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub jump_force: f32,
    pub player_height: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window: WindowConfig {
                width: 1280,
                height: 720,
                title: String::from("Cubix"),
                fullscreen: true,
                vsync: true,
            },
            camera: CameraConfig {
                sensitivity: 0.1,
                fov: 70.0,
                near_plane: 0.1,
                far_plane: 1000.0,
                position_x: 8.0,
                position_y: 8.0, // Start slightly above terrain
                position_z: 8.0,
            },
            controls: ControlsConfig {
                cursor_locked: true,
            },
            physics: PhysicsConfig {
                gravity: 0.015,
                jump_force: 0.2,
                player_height: 1.8,
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.toml";

        // If config file exists, load it
        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Error parsing config file: {}", e);
                        eprintln!("Using default config instead");
                    }
                },
                Err(e) => {
                    eprintln!("Error reading config file: {}", e);
                    eprintln!("Using default config instead");
                }
            }
        }

        // Create default config if not exists
        let config = Config::default();
        config.save();
        config
    }

    pub fn save(&self) {
        let config_str = toml::to_string_pretty(self).expect("Failed to serialize config");
        let mut file = File::create("config.toml").expect("Failed to create config file");
        file.write_all(config_str.as_bytes())
            .expect("Failed to write config file");
    }
}
