use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
