mod camera;
mod collision;
mod components;
mod movement;
mod setup;

// Re-export everything
pub use camera::*;
pub use collision::*;
pub use movement::*;
pub use setup::*;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, setup_cursor))
            // Apply gravity first, then handle collisions, then process movement
            .add_systems(
                Update,
                (
                    apply_gravity,
                    handle_collisions,
                    player_movement,
                    mouse_look,
                )
                    .chain(),
            ); // Chain ensures they run in this order
    }
}
