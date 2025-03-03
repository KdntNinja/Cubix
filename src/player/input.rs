use crate::config::Config;
use crate::player::camera::Camera;
use cgmath::{InnerSpace, Vector3};
use glfw::{Action, Key, Window};
use std::collections::HashMap;

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

    pub fn process_input(
        &mut self,
        _window: &Window,
        camera: &mut Camera,
        delta_time: f32,
        config: &Config,
    ) {
        // Update timers
        self.last_jump_time += delta_time;

        // Set movement speed
        let base_speed = if self.is_key_pressed(Key::LeftControl) {
            10.0 // Sprint speed
        } else {
            5.0 // Normal speed
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

    fn is_key_pressed(&self, key: Key) -> bool {
        *self.key_states.get(&key).unwrap_or(&false)
    }

    fn move_camera(&self, camera: &mut Camera, offset: Vector3<f32>) {
        // In non-fly mode, only move horizontally
        if !self.fly_mode {
            camera.position.x += offset.x;
            camera.position.z += offset.z;
        } else {
            camera.position += offset;
        }
    }

    // Block placement and breaking methods
    pub fn _place_block(&self) -> bool {
        // Will be implemented when world editing is added
        false
    }

    pub fn _break_block(&self) -> bool {
        // Will be implemented when world editing is added
        false
    }
}

// Function to register input handling in the main event loop
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
