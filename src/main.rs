extern crate gl;
extern crate glfw;

use cgmath::Matrix;
use gl::types::*;
use glfw::Context;

mod events;
mod rendering;
mod world;

use events::process_events;
use world::init::App;

fn main() {
    let mut app = App::new(600, 480, "Multiple Cubes");

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        // Render loop
        while !app.window.should_close() {
            process_events(&mut app.window, &app.events);

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
            //draw_cubes(&app.mesh, &app.shader, app.glfw.get_time() as f32);

            app.window.swap_buffers();
            app.glfw.poll_events();
        }
    }
}
