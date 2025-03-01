use super::components::Settings;
use config::{Config, ConfigError, File};
use std::{fs, path::Path};

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
