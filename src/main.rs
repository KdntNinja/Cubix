extern crate gl;
extern crate glfw;

use cgmath::Matrix;
use gl::types::*;
use glfw::Context;
use std::time::{Duration, Instant};

mod config;
mod events;
mod player;
mod rendering;
mod world;

use crate::config::Config;
use crate::events::process_events;
use crate::player::input::{PlayerInput, handle_movement_input};
use crate::world::init::App;

fn main() {
    // Load config
    let mut config = Config::load();

    let mut app = App::new(&config);

    // Initialize player input system
    let mut player_input = PlayerInput::new(&config);

    // For calculating delta time
    let mut last_frame = Instant::now();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        // Render loop
        while !app.window.should_close() {
            // Calculate delta time
            let current_frame = Instant::now();
            let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
            last_frame = current_frame;

            // Process events and update projection if needed
            if let Some(new_projection) = process_events(
                &mut app.window,
                &app.events,
                &mut app.camera,
                &mut config,
                &mut app.glfw,
                &mut player_input,
            ) {
                app.projection = new_projection;
            }

            // Handle player movement with delta time
            handle_movement_input(
                &app.window,
                &mut app.camera,
                &mut player_input,
                delta_time,
                &config,
            );

            // Update the view matrix with new camera orientation
            app.update_view_matrix();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            app.shader.use_program();

            //Set up projection matrix
            let projection_location =
                gl::GetUniformLocation(app.shader.id, b"projection\0".as_ptr() as *const GLchar);
            gl::UniformMatrix4fv(
                projection_location,
                1,
                gl::FALSE as GLboolean,
                app.projection.as_ptr(),
            );

            //Set up view matrix
            let view_location =
                gl::GetUniformLocation(app.shader.id, b"view\0".as_ptr() as *const GLchar);
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE as GLboolean, app.view.as_ptr());

            app.world.draw(&app.shader, app.glfw.get_time() as f32);

            app.window.swap_buffers();
            app.glfw.poll_events();

            // Cap framerate (optional)
            if config.window.vsync {
                std::thread::sleep(Duration::from_millis(16));
            }
        }
    }
}
