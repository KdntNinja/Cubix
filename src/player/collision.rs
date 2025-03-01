use super::components::Player;
use crate::blocks::Block;
use crate::settings::Settings;
use bevy::prelude::*;

pub fn handle_collisions(
    settings: Res<Settings>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    block_query: Query<&Transform, (With<Block>, Without<Player>)>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let player_pos = player_transform.translation;
        let player_half_height = 0.9;
        let player_half_width = 0.4;
        let block_size = settings.world.block_size;

        // Reset grounded status but check for ground below first
        let was_grounded = player.grounded;
        player.grounded = false;

        // Ground detection with a more lenient buffer
        let ground_check_distance = 0.1; // Increased from 0.05

        // First pass: check for ground
        for block_transform in block_query.iter() {
            let block_pos = block_transform.translation;

            // Check if player is above this block within ground detection range
            let dx = (player_pos.x - block_pos.x).abs();
            let dz = (player_pos.z - block_pos.z).abs();

            if dx < (player_half_width + block_size / 2.0 - 0.05)
                && dz < (player_half_width + block_size / 2.0 - 0.05)
                && player_pos.y > block_pos.y
                && (player_pos.y - player_half_height - block_pos.y - block_size / 2.0).abs()
                    <= ground_check_distance
            {
                player.grounded = true;

                // Only snap to ground if moving down
                if player.velocity.y <= 0.0 {
                    player_transform.translation.y =
                        block_pos.y + block_size / 2.0 + player_half_height + 0.001;
                    player.velocity.y = 0.0;
                }
                break; // Found ground, no need to check further blocks for grounding
            }
        }

        // Second pass: general collision resolution
        for block_transform in block_query.iter() {
            let block_pos = block_transform.translation;

            // Skip detailed collision if we're clearly not near this block
            let distance = player_pos.distance(block_pos);
            if distance > player_half_height + player_half_width + block_size {
                continue;
            }

            // Check collision with slight reduction in size for smoother movement
            let dx = (player_pos.x - block_pos.x).abs();
            let dy = (player_pos.y - block_pos.y).abs();
            let dz = (player_pos.z - block_pos.z).abs();

            let collision_x = dx < (player_half_width + block_size / 2.0);
            let collision_y = dy < (player_half_height + block_size / 2.0);
            let collision_z = dz < (player_half_width + block_size / 2.0);

            // Collision detected
            if collision_x && collision_y && collision_z {
                // Find collision direction (smallest penetration)
                let pen_x = player_half_width + block_size / 2.0 - dx;
                let pen_y = player_half_height + block_size / 2.0 - dy;
                let pen_z = player_half_width + block_size / 2.0 - dz;

                // Resolve with slight priority to Y axis (better for stairs/slopes)
                if pen_y <= pen_x * 1.2 && pen_y <= pen_z * 1.2 {
                    // Y-axis collision
                    if player_pos.y > block_pos.y {
                        player_transform.translation.y += pen_y;
                        player.grounded = true;
                    } else {
                        player_transform.translation.y -= pen_y;
                    }
                    player.velocity.y = 0.0;
                } else if pen_x <= pen_z {
                    // X-axis collision
                    if player_pos.x > block_pos.x {
                        player_transform.translation.x += pen_x;
                    } else {
                        player_transform.translation.x -= pen_x;
                    }
                    player.velocity.x = 0.0;
                } else {
                    // Z-axis collision
                    if player_pos.z > block_pos.z {
                        player_transform.translation.z += pen_z;
                    } else {
                        player_transform.translation.z -= pen_z;
                    }
                    player.velocity.z = 0.0;
                }
            }
        }

        // Short coyote time after leaving ground
        if was_grounded && !player.grounded && player.velocity.y <= 0.0 {
            player.grounded = true;
            player.velocity.y = 0.0;
        }
    }
}
