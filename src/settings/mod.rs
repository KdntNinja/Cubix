mod components;
mod loading;

// Re-export everything
pub use components::*;

use bevy::prelude::*;

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
