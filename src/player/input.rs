use crate::config::Config;
use glfw::{Action, Key};
use std::collections::HashMap;

/// Manages player input, including movement and key states.
pub struct PlayerInput {
    key_states: HashMap<Key, bool>,
    pub movement_speed: f32,
    pub fly_mode: bool,
    pub gravity: f32,
    pub jump_force: f32,
    pub velocity: cgmath::Vector3<f32>,
    pub on_ground: bool,
    pub last_jump_time: f32,
    pub jump_cooldown: f32,
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
            velocity: cgmath::Vector3::new(0.0, 0.0, 0.0),
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

    /// Checks if a key is currently pressed.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check.
    ///
    /// # Returns
    ///
    /// `true` if the key is pressed, `false` otherwise.
    pub fn is_key_pressed(&self, key: Key) -> bool {
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
