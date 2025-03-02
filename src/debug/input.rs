use super::components::DebugState;
use bevy::prelude::*;

pub fn handle_debug_keys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
) {
    // Use F1 to toggle the entire debug overlay (like Minecraft's F3)
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_state.show_debug = !debug_state.show_debug;
    }
}
