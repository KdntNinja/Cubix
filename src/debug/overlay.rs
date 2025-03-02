use super::components::*;
use crate::player::components::Player;
use crate::settings::Settings;
use bevy::prelude::*;

pub fn setup_debug_ui(mut commands: Commands) {
    // Main debug container (initially hidden)
    commands
        .spawn((
            Text::from(""), // Empty parent
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Hidden,
            DebugContainer,
        ))
        .with_children(|parent| {
            // FPS text (top left)
            parent.spawn((
                Text::from("FPS: 0 (0.0ms)"),
                Transform::from_xyz(5.0, 5.0, 0.0),
                FpsText,
            ));

            // Player info (below FPS)
            parent.spawn((
                Text::from("XYZ: 0.0, 0.0, 0.0\nFacing: North\nBlock: 0, 0, 0"),
                Transform::from_xyz(5.0, 40.0, 0.0),
                PlayerInfoText,
            ));

            // World info (below player info)
            parent.spawn((
                Text::from("World Info: Not Available"),
                Transform::from_xyz(5.0, 100.0, 0.0),
                WorldInfoText,
            ));

            // Right side - controls
            parent.spawn((
                Text::from("F1: Toggle Debug | WASD: Move | Space: Jump | Shift: Sprint"),
                Transform::from_xyz(400.0, 5.0, 0.0),
                KeybindText,
            ));
        });
}

pub fn update_debug_info(
    debug_state: Res<DebugState>,
    settings: Res<Settings>,
    player_query: Query<(&Transform, &Player)>,
    mut container_query: Query<&mut Visibility, With<DebugContainer>>,
    mut player_info_query: Query<&mut Text, With<PlayerInfoText>>,
    mut world_info_query: Query<&mut Text, With<WorldInfoText>>,
) {
    // Update main container visibility
    if let Ok(mut visibility) = container_query.get_single_mut() {
        *visibility = if debug_state.show_debug {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // Only update text content if debug mode is active
    if debug_state.show_debug {
        // Update player info in Minecraft style
        if let Ok(mut text) = player_info_query.get_single_mut() {
            if let Ok((transform, player)) = player_query.get_single() {
                *text = Text::from(format!(
                    "XYZ: {:.2}, {:.2}, {:.2}\n\
                         Block: {}, {}, {}\n\
                         Chunk: {}, {}\n\
                         Facing: {}\n\
                         Speed: {:.2} m/s\n\
                         Grounded: {}",
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                    transform.translation.x.floor(),
                    transform.translation.y.floor(),
                    transform.translation.z.floor(),
                    (transform.translation.x / 16.0).floor(),
                    (transform.translation.z / 16.0).floor(),
                    get_facing_direction(transform),
                    player.velocity.length(),
                    player.grounded
                ));
            }
        }

        // Update world info using memory values from debug_state
        if let Ok(mut text) = world_info_query.get_single_mut() {
            *text = Text::from(format!(
                "Block Size: {:.1}\n\
                     Chunk Size: {}\n\
                     Gravity: {:.1}\n\
                     Memory: {:.1}% ({:.1} MB / {:.0} MB)",
                settings.world.block_size,
                settings.world.chunk_size,
                settings.player.gravity,
                debug_state.current_memory_usage,
                debug_state.current_process_memory,
                debug_state.current_total_memory
            ));
        }
    }
}

// Helper to get player direction like Minecraft
fn get_facing_direction(transform: &Transform) -> &'static str {
    let forward = transform.forward();
    let angle = forward.z.atan2(forward.x);
    let degrees = (angle.to_degrees() + 180.0) % 360.0;

    match degrees as u32 {
        315..=359 | 0..=45 => "East (+X)",
        46..=135 => "South (+Z)",
        136..=225 => "West (-X)",
        _ => "North (-Z)",
    }
}
