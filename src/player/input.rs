use crate::config::Config;
use crate::player::camera::Camera;
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
    pub fn process_input(
        &mut self,
        _window: &Window,
        camera: &mut Camera,
        delta_time: f32,
        config: &Config,
    ) {
        // Update timers
        self.last_jump_time += delta_time;

        // Set movement speed - higher speed for fly mode
        let base_speed = if self.is_key_pressed(Key::LeftControl) {
            15.0 // Sprint speed
        } else if self.fly_mode {
            8.0 // Higher base speed for fly mode
        } else {
            5.0 // Normal walking speed
        };

        // Calculate frame-dependent speed
        let speed = base_speed * delta_time;
        self.movement_speed = base_speed;

        // Forward/backward movement (only horizontal component in walking mode)
        if self.is_key_pressed(Key::W) {
            let mut movement = camera.front;
            if !self.fly_mode {
                movement.y = 0.0; // Remove vertical component in walking mode
                if movement.magnitude() > 0.0001 {
                    movement = movement.normalize() * speed;
                }
            } else {
                movement = movement * speed;
            }
            self.move_camera(camera, movement);
        }

        if self.is_key_pressed(Key::S) {
            let mut movement = -camera.front;
            if !self.fly_mode {
                movement.y = 0.0; // Remove vertical component in walking mode
                if movement.magnitude() > 0.0001 {
                    movement = movement.normalize() * speed;
                }
            } else {
                movement = movement * speed;
            }
            self.move_camera(camera, movement);
        }

        // Left/right movement (strafe)
        if self.is_key_pressed(Key::A) || self.is_key_pressed(Key::D) {
            let right = camera.front.cross(camera.up).normalize();
            let direction = if self.is_key_pressed(Key::A) {
                -1.0
            } else {
                1.0
            };
            self.move_camera(camera, right * direction * speed);
        }

        // Handle vertical movement based on mode
        if self.fly_mode {
            // Flying controls - direct up/down movement
            if self.is_key_pressed(Key::Space) {
                self.move_camera(camera, Vector3::new(0.0, speed, 0.0));
            }
            if self.is_key_pressed(Key::LeftShift) {
                self.move_camera(camera, Vector3::new(0.0, -speed, 0.0));
            }
        } else {
            // Walking mode with physics

            // Jumping with improved feel
            if self.is_key_pressed(Key::Space)
                && self.on_ground
                && self.last_jump_time > self.jump_cooldown
            {
                self.velocity.y = self.jump_force;
                self.on_ground = false;
                self.last_jump_time = 0.0;
            }

            // Apply gravity with delta time scaling
            self.velocity.y -= self.gravity * delta_time * 60.0; // Scale with target 60fps

            // Terminal velocity cap to prevent falling too fast
            if self.velocity.y < -0.8 {
                self.velocity.y = -0.8;
            }

            // Apply vertical velocity
            camera.position.y += self.velocity.y;

            // Ground collision detection
            if camera.position.y <= config.physics.player_height {
                camera.position.y = config.physics.player_height;
                self.velocity.y = 0.0;
                self.on_ground = true;
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

    /// Moves the camera by the given offset.
    ///
    /// # Arguments
    ///
    /// * `camera` - A mutable reference to the player's camera.
    /// * `offset` - The offset to move the camera by.
    fn move_camera(&self, camera: &mut Camera, offset: Vector3<f32>) {
        // Check for NaN or infinite values in the offset
        if !offset.x.is_finite() || !offset.y.is_finite() || !offset.z.is_finite() {
            return; // Reject invalid movements
        }

        // In non-fly mode, only move horizontally
        if !self.fly_mode {
            camera.position.x += offset.x;
            camera.position.z += offset.z;
        } else {
            camera.position += offset;
        }
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
) {
    // Skip input processing if cursor isn't locked (in menus)
    if !config.controls.cursor_locked {
        return;
    }

    player_input.process_input(window, camera, delta_time, config);
}
