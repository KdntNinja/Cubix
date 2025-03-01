mod components;
mod generation;
mod interaction;
mod materials;

// Re-export everything
pub use components::*;
pub use generation::*;
pub use interaction::*;
pub use materials::*;

use bevy::prelude::*;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_block_materials)
            .add_systems(Update, highlight_hovered_block);
    }
}
