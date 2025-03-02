mod components;
mod graphs;
mod input;
mod overlay;
mod performance;

pub use components::*;
pub use graphs::*;
pub use input::*;
pub use overlay::*;
pub use performance::*;

use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_debug_ui, setup_performance_graphs))
            .add_systems(
                Update,
                (
                    handle_debug_keys,
                    update_fps_counter,
                    update_performance_graphs,
                )
                    .before(update_debug_info),
            )
            .add_systems(Update, update_debug_info)
            .insert_resource(DebugState {
                show_debug: false,
                fps_history: Vec::with_capacity(100),
                memory_usage: Vec::with_capacity(100),
                last_update: 0.0,
                current_memory_usage: 0.0,
                current_process_memory: 0.0,
                current_total_memory: 0.0,
            });
    }
}
