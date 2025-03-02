use bevy::prelude::*;

#[derive(Resource)]
pub struct DebugState {
    pub show_debug: bool,
    pub fps_history: Vec<f32>,
    pub memory_usage: Vec<f32>,
    pub last_update: f32,
    // New fields to store memory information
    pub current_memory_usage: f32,
    pub current_process_memory: f32,
    pub current_total_memory: f32,
}

// Component markers
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct PlayerInfoText;

#[derive(Component)]
pub struct WorldInfoText;

#[derive(Component)]
pub struct KeybindText;

#[derive(Component)]
pub struct FpsGraph;

#[derive(Component)]
pub struct MemoryGraph;

#[derive(Component)]
pub struct DebugContainer;
