use super::components::Settings;
use config::ConfigError;
use log::info;

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        info!("Using default settings (no file I/O)");

        // Return default settings
        Ok(Self::default())
    }
}
