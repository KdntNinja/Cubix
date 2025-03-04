use crate::config::Config;
use crate::player::camera::Camera;
use crate::player::input::PlayerInput;
use crate::world::world::World;
use cgmath::Point3;
use cgmath::{InnerSpace, Vector3};
use glfw::Window;

/// Processes player input and updates the camera position.
///
/// # Arguments
///
/// * `_window` - A reference to the GLFW window.
/// * `camera` - A mutable reference to the player's camera.
/// * `delta_time` - The time elapsed since the last frame.
/// * `config` - A reference to the game configuration.
/// * `world` - A reference to the game world.
pub fn process_input(
    player_input: &mut PlayerInput,
    _window: &Window,
    camera: &mut Camera,
    delta_time: f32,
    config: &Config,
    world: &World,
) {
    // Update timers
    player_input.last_jump_time += delta_time;

    // Set movement speed
    let base_speed = if player_input.is_key_pressed(Key::LeftControl) {
        15.0 // Sprint speed
    } else if player_input.fly_mode {
        8.0 // Fly mode speed
    } else {
        5.0 // Normal walking speed
    };

    // Calculate frame-dependent speed
    let speed = base_speed * delta_time;
    player_input.movement_speed = base_speed;

    // Player collision properties
    let player_radius = 0.3; // Reasonable radius for player
    let player_height = config.physics.player_height;

    // Store current position before movement
    let current_pos = camera.position;
    let mut target_pos = current_pos;

    // Ground detection for jumping
    if !player_input.fly_mode {
        // Position is now the camera position (eyes), so we check at feet level
        let feet_position = Point3::new(
            camera.position.x,
            camera.position.y - config.physics.player_height,
            camera.position.z,
        );

        // Check if there's a block below feet
        let ground_check_pos = Point3::new(
            feet_position.x,
            feet_position.y - 0.05, // Check slightly below feet
            feet_position.z,
        );

        let was_on_ground = player_input.on_ground;
        player_input.on_ground = world.check_collision(&ground_check_pos, player_radius, 0.1);

        // If we just landed, reset vertical velocity
        if !was_on_ground && player_input.on_ground {
            player_input.velocity.y = 0.0;
        }
    }

    // Left/right movement (strafe)
    if player_input.is_key_pressed(Key::A) || player_input.is_key_pressed(Key::D) {
        let right = camera.front.cross(camera.up).normalize();
        let direction = if player_input.is_key_pressed(Key::A) {
            -1.0
        } else {
            1.0
        };
        target_pos += right * direction * speed;
    }

    // Forward/backward movement
    if player_input.is_key_pressed(Key::W) || player_input.is_key_pressed(Key::S) {
        // Get horizontal component of camera front vector
        let mut forward = camera.front;
        if !player_input.fly_mode {
            // When not flying, we move only horizontally
            forward.y = 0.0;
            // Only normalize if the vector isn't zero length
            if forward.magnitude() > 0.00001 {
                forward = forward.normalize();
            }
        }

        let direction = if player_input.is_key_pressed(Key::S) {
            -1.0
        } else {
            1.0
        };
        target_pos += forward * direction * speed;
    }

    // Handle vertical movement based on mode
    if player_input.fly_mode {
        // Flying controls - direct up/down movement
        if player_input.is_key_pressed(Key::Space) {
            target_pos.y += speed;
        }
        if player_input.is_key_pressed(Key::LeftShift) {
            target_pos.y -= speed;
        }
    } else {
        // Walking mode with physics and jumping
        if player_input.is_key_pressed(Key::Space)
            && player_input.on_ground
            && player_input.last_jump_time > player_input.jump_cooldown
        {
            player_input.velocity.y = player_input.jump_force;
            player_input.on_ground = false;
            player_input.last_jump_time = 0.0;
        }

        // Apply gravity
        player_input.velocity.y -= player_input.gravity * delta_time * 60.0;

        // Terminal velocity cap
        if player_input.velocity.y < -0.8 {
            player_input.velocity.y = -0.8;
        }

        // Apply vertical velocity
        target_pos.y += player_input.velocity.y;
    }

    // Resolve collisions with world
    camera.position =
        world.resolve_collision(current_pos, target_pos, player_radius, player_height);

    // Ground detection for jumping
    if !player_input.fly_mode {
        // Check if there's a block below us
        let ground_check_pos = Point3::new(
            camera.position.x,
            camera.position.y - 0.05, // Check slightly below feet
            camera.position.z,
        );

        let was_on_ground = player_input.on_ground;
        player_input.on_ground = world.check_collision(&ground_check_pos, player_radius, 0.1);

        // If we just landed, reset vertical velocity
        if !was_on_ground && player_input.on_ground {
            player_input.velocity.y = 0.0;
        }
    }
}

/// Handles player movement input in the main event loop.
///
/// # Arguments
///
/// * `window` - A reference to the GLFW window.
/// * `camera` - A mutable reference to the player's camera.
/// * `player_input` - A mutable reference to the `PlayerInput` instance.
/// * `delta_time` - The time elapsed since the last frame.
/// * `config` - A reference to the game configuration.
/// * `world` - A reference to the game world.
pub fn handle_movement_input(
    window: &Window,
    camera: &mut Camera,
    player_input: &mut PlayerInput,
    delta_time: f32,
    config: &Config,
    world: &World,
) {
    // Skip input processing if cursor isn't locked (in menus)
    if !config.controls.cursor_locked {
        return;
    }

    process_input(player_input, window, camera, delta_time, config, world);
}