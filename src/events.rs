use crate::config::Config;
use crate::player::input::PlayerInput;
use cgmath::Matrix4;
use cgmath::{Deg, perspective};
use gl;
use glfw::{Action, Glfw, GlfwReceiver, Key, Window, WindowEvent};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global mutable state wrapped in thread-safe containers
static MOUSE_STATE: Lazy<Mutex<MouseState>> = Lazy::new(|| {
    Mutex::new(MouseState {
        last_x: 300.0,
        last_y: 240.0,
        first_mouse: true,
    })
});

struct MouseState {
    last_x: f32,
    last_y: f32,
    first_mouse: bool,
}

pub fn process_events(
    window: &mut Window,
    events: &GlfwReceiver<(f64, WindowEvent)>,
    camera: &mut crate::player::camera::Camera,
    config: &mut Config,
    glfw: &mut Glfw,
    player_input: &mut PlayerInput,
) -> Option<Matrix4<f32>> {
    let mut new_projection = None;

    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
                // Recalculate projection matrix to maintain aspect ratio
                new_projection = Some(perspective(
                    Deg(config.camera.fov),
                    width as f32 / height as f32,
                    config.camera.near_plane,
                    config.camera.far_plane,
                ));
            }
            WindowEvent::Key(key, _, action, _) => {
                // Special keys handling
                match (key, action) {
                    (Key::Escape, Action::Press) => {
                        // Make the window close when Escape is pressed
                        window.set_should_close(true)
                    }
                    (Key::F11, Action::Press) => {
                        // Toggle fullscreen when F11 is pressed
                        config.window.fullscreen = !config.window.fullscreen;

                        if config.window.fullscreen {
                            // Save window properties before going fullscreen
                            let (width, height) = window.get_size();
                            config.window.width = width as u32;
                            config.window.height = height as u32;

                            // Switch to fullscreen using a direct approach
                            glfw.with_primary_monitor(|_, m| {
                                if let Some(monitor) = m {
                                    if let Some(video_mode) = monitor.get_video_mode() {
                                        // Use video mode resolution for fullscreen
                                        window.set_monitor(
                                            glfw::WindowMode::FullScreen(monitor),
                                            0,
                                            0,
                                            video_mode.width,
                                            video_mode.height,
                                            Some(video_mode.refresh_rate),
                                        );

                                        // Force update viewport size
                                        unsafe {
                                            gl::Viewport(
                                                0,
                                                0,
                                                video_mode.width as i32,
                                                video_mode.height as i32,
                                            );
                                        }

                                        // Update projection for new resolution
                                        new_projection = Some(perspective(
                                            Deg(config.camera.fov),
                                            video_mode.width as f32 / video_mode.height as f32,
                                            config.camera.near_plane,
                                            config.camera.far_plane,
                                        ));
                                    }
                                }
                            });
                        } else {
                            // Restore windowed mode with the original dimensions
                            window.set_monitor(
                                glfw::WindowMode::Windowed,
                                100, // Default position
                                100, // Default position
                                config.window.width,
                                config.window.height,
                                None,
                            );

                            // Explicitly update the viewport and projection matrix with the windowed dimensions
                            unsafe {
                                gl::Viewport(
                                    0,
                                    0,
                                    config.window.width as i32,
                                    config.window.height as i32,
                                );
                            }

                            // Update projection for restored window size
                            new_projection = Some(perspective(
                                Deg(config.camera.fov),
                                config.window.width as f32 / config.window.height as f32,
                                config.camera.near_plane,
                                config.camera.far_plane,
                            ));
                        }

                        // Save config changes
                        config.save();
                    }
                    _ => {
                        // Pass other keys to player input system
                        player_input.key_callback(key, action);
                    }
                }
            }
            WindowEvent::CursorPos(x_pos, y_pos) => {
                // Only process mouse movement when cursor is locked
                if !config.controls.cursor_locked {
                    continue;
                }

                let (x_pos, y_pos) = (x_pos as f32, y_pos as f32);
                let mut mouse_state = MOUSE_STATE.lock().unwrap();

                if mouse_state.first_mouse {
                    mouse_state.last_x = x_pos;
                    mouse_state.last_y = y_pos;
                    mouse_state.first_mouse = false;
                    continue; // Skip this first frame to avoid large jumps
                }

                // Limit the maximum movement per frame to avoid huge jumps
                let mut x_offset = x_pos - mouse_state.last_x;
                let mut y_offset = mouse_state.last_y - y_pos;

                // Constrain maximum movement per frame
                const MAX_MOVEMENT: f32 = 100.0;
                x_offset = x_offset.max(-MAX_MOVEMENT).min(MAX_MOVEMENT);
                y_offset = y_offset.max(-MAX_MOVEMENT).min(MAX_MOVEMENT);

                mouse_state.last_x = x_pos;
                mouse_state.last_y = y_pos;

                camera.process_mouse_movement(x_offset, y_offset, true, config.camera.sensitivity);
            }
            WindowEvent::MouseButton(_button, _action, _) => {
                if !config.controls.cursor_locked {
                    continue;
                }
                // Implementation will come later
            }
            _ => {}
        }
    }
    new_projection
}
