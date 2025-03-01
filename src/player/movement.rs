use super::components::Player;
use crate::settings::Settings;
use bevy::prelude::*;

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let delta = time.delta_secs();

        // Get input direction
        if keyboard.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Check for sprint
        let sprint_multiplier = if keyboard.pressed(KeyCode::ControlLeft) {
            1.6
        } else {
            1.0
        };

        // Transform movement direction based on player rotation
        let (forward, right) = (transform.forward(), transform.right());
        let desired_velocity = if direction != Vec3::ZERO {
            let normalized = (forward * -direction.z + right * direction.x).normalize();
            Vec3::new(
                normalized.x * settings.player.speed * sprint_multiplier,
                0.0,
                normalized.z * settings.player.speed * sprint_multiplier,
            )
        } else {
            Vec3::ZERO
        };

        // Handle jumping with a small buffer for better responsiveness
        if keyboard.pressed(KeyCode::Space) && player.grounded {
            player.velocity.y = settings.player.jump_force;
            player.grounded = false;
        }

        // Apply different acceleration depending on grounded state
        let acceleration = if player.grounded { 10.0 } else { 2.0 };
        let friction = if player.grounded { 8.0 } else { 0.5 };

        // Horizontal acceleration toward desired velocity
        if direction != Vec3::ZERO {
            // Accelerate toward desired velocity
            player.velocity.x = lerp(player.velocity.x, desired_velocity.x, delta * acceleration);
            player.velocity.z = lerp(player.velocity.z, desired_velocity.z, delta * acceleration);
        } else if player.grounded {
            // Apply friction when no input and on ground
            player.velocity.x = dampen(player.velocity.x, delta * friction);
            player.velocity.z = dampen(player.velocity.z, delta * friction);
        }

        // Apply velocity
        transform.translation += player.velocity * delta;
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    settings: Res<Settings>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if !player.grounded {
            player.velocity.y -= settings.player.gravity * time.delta_secs();
        }
    }
}

// Utility functions
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.min(1.0)
}

pub fn dampen(value: f32, factor: f32) -> f32 {
    if value.abs() < 0.001 {
        0.0
    } else {
        value * (1.0 - factor).max(0.0)
    }
}
