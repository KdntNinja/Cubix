use crate::config::Config;
use crate::player::camera::Camera;
use crate::world::world::World;
use cgmath::Point3;
use cgmath::{InnerSpace, Vector3};
use glfw::{Action, Key, Window};
use std::collections::HashMap;

/// Manages player input, including movement and key states.
pub struct PlayerInput {
    key_states: HashMap<Key, bool>,
    movement_speed: f32,
    fly_mode: bool,
    gravity: f32,
    jump_force: f32,
    velocity: Vector3<f32>,
    on_ground: bool,
    last_jump_time: f32,
    jump_cooldown: f32,
}

impl PlayerInput {
    /// Creates a new `PlayerInput` instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the game configuration.
    ///
    /// # Returns
    ///
    /// A new `PlayerInput` instance.
    pub fn new(config: &Config) -> Self {
        PlayerInput {
            key_states: HashMap::new(),
            movement_speed: 5.0,
            fly_mode: false,
            gravity: config.physics.gravity,
            jump_force: config.physics.jump_force,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            on_ground: false,
            last_jump_time: 0.0,
            jump_cooldown: 0.2, // Prevent jump spam
        }
    }

    /// Handles key press and release events.
    ///
    /// # Arguments
    ///
    /// * `key` - The key that was pressed or released.
    /// * `action` - The action (press or release) associated with the key.
    pub fn key_callback(&mut self, key: Key, action: Action) {
        match action {
            Action::Press => {
                self.key_states.insert(key, true);

                // Toggle fly mode with F key
                if key == Key::F {
                    self.fly_mode = !self.fly_mode;
                    if self.fly_mode {
                        self.velocity.y = 0.0; // Reset vertical velocity when entering fly mode
                    }
                }
            }
            Action::Release => {
                self.key_states.insert(key, false);
            }
            _ => {}
        }
    }

    /// Processes player input and updates the camera position.
    ///
    /// # Arguments
    ///
    /// * `_window` - A reference to the GLFW window.
    /// * `camera` - A mutable reference to the player's camera.
    /// * `delta_time` - The time elapsed since the last frame.
    /// * `config` - A reference to the game configuration.
    // Add these imports at the top of src/player/input.rs
    pub fn process_input(
        &mut self,
        _window: &Window,
        camera: &mut Camera,
        delta_time: f32,
        config: &Config,
        world: &World, // New parameter
    ) {
        // Update timers
        self.last_jump_time += delta_time;

        // Set movement speed
        let base_speed = if self.is_key_pressed(Key::LeftControl) {
            15.0 // Sprint speed
        } else if self.fly_mode {
            8.0 // Fly mode speed
        } else {
            5.0 // Normal walking speed
        };

        // Calculate frame-dependent speed
        let speed = base_speed * delta_time;
        self.movement_speed = base_speed;

        // Player collision properties
        let player_radius = 0.3; // Reasonable radius for player
        let player_height = config.physics.player_height;

        // Store current position before movement
        let current_pos = camera.position;
        let mut target_pos = current_pos;

        // Ground detection for jumping
        if !self.fly_mode {
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

            let was_on_ground = self.on_ground;
            self.on_ground = world.check_collision(&ground_check_pos, player_radius, 0.1);

            // If we just landed, reset vertical velocity
            if !was_on_ground && self.on_ground {
                self.velocity.y = 0.0;
            }
        }

        // Left/right movement (strafe)
        if self.is_key_pressed(Key::A) || self.is_key_pressed(Key::D) {
            let right = camera.front.cross(camera.up).normalize();
            let direction = if self.is_key_pressed(Key::A) {
                -1.0
            } else {
                1.0
            };
            target_pos += right * direction * speed;
        }

        // Forward/backward movement
        if self.is_key_pressed(Key::W) || self.is_key_pressed(Key::S) {
            // Get horizontal component of camera front vector
            let mut forward = camera.front;
            if !self.fly_mode {
                // When not flying, we move only horizontally
                forward.y = 0.0;
                // Only normalize if the vector isn't zero length
                if forward.magnitude() > 0.00001 {
                    forward = forward.normalize();
                }
            }

            let direction = if self.is_key_pressed(Key::S) {
                -1.0
            } else {
                1.0
            };
            target_pos += forward * direction * speed;
        }

        // Handle vertical movement based on mode
        if self.fly_mode {
            // Flying controls - direct up/down movement
            if self.is_key_pressed(Key::Space) {
                target_pos.y += speed;
            }
            if self.is_key_pressed(Key::LeftShift) {
                target_pos.y -= speed;
            }
        } else {
            // Walking mode with physics and jumping
            if self.is_key_pressed(Key::Space)
                && self.on_ground
                && self.last_jump_time > self.jump_cooldown
            {
                self.velocity.y = self.jump_force;
                self.on_ground = false;
                self.last_jump_time = 0.0;
            }

            // Apply gravity
            self.velocity.y -= self.gravity * delta_time * 60.0;

            // Terminal velocity cap
            if self.velocity.y < -0.8 {
                self.velocity.y = -0.8;
            }

            // Apply vertical velocity
            target_pos.y += self.velocity.y;
        }

        // Resolve collisions with world
        camera.position =
            world.resolve_collision(current_pos, target_pos, player_radius, player_height);

        // Ground detection for jumping
        if !self.fly_mode {
            // Check if there's a block below us
            let ground_check_pos = Point3::new(
                camera.position.x,
                camera.position.y - 0.05, // Check slightly below feet
                camera.position.z,
            );

            let was_on_ground = self.on_ground;
            self.on_ground = world.check_collision(&ground_check_pos, player_radius, 0.1);

            // If we just landed, reset vertical velocity
            if !was_on_ground && self.on_ground {
                self.velocity.y = 0.0;
            }
        }
    }
    /// Checks if a key is currently pressed.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check.
    ///
    /// # Returns
    ///
    /// `true` if the key is pressed, `false` otherwise.
    fn is_key_pressed(&self, key: Key) -> bool {
        *self.key_states.get(&key).unwrap_or(&false)
    }

    /// Placeholder method for block placement.
    ///
    /// # Returns
    ///
    /// `false` as block placement is not yet implemented.
    pub fn _place_block(&self) -> bool {
        false
    }

    /// Placeholder method for block breaking.
    ///
    /// # Returns
    ///
    /// `false` as block breaking is not yet implemented.
    pub fn _break_block(&self) -> bool {
        false
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
pub fn handle_movement_input(
    window: &Window,
    camera: &mut Camera,
    player_input: &mut PlayerInput,
    delta_time: f32,
    config: &Config,
    world: &World, // New parameter
) {
    // Skip input processing if cursor isn't locked (in menus)
    if !config.controls.cursor_locked {
        return;
    }

    player_input.process_input(window, camera, delta_time, config, world);
}
