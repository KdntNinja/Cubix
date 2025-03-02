extern crate gl;
extern crate glfw;

use cgmath::{Deg, InnerSpace, Matrix, Matrix4, Point3, Vector3, perspective};
use gl::types::*;
use glfw::Context;

mod events;
mod rendering;

use events::process_events;
use rendering::mesh::Mesh;
use rendering::shader::Shader;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(600, 480, "Multiple Cubes", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Vertex data for a single cube
    let cube_vertices: [f32; 108] = [
        -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5,
        -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
        -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5,
        0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
        -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5,
        0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
    ];

    let cube_count = 3;

    let shader = Shader::new(
        "src/shaders/vertex_shader.glsl",
        "src/shaders/fragment_shader.glsl",
    );
    let mesh = Mesh::new(&cube_vertices);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        // Render loop
        while !window.should_close() {
            process_events(&mut window, &events);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

            // Set up projection matrix
            let projection_location =
                gl::GetUniformLocation(shader.id, b"projection\0".as_ptr() as *const GLchar);
            let projection: Matrix4<f32> = perspective(Deg(45.0), 600.0 / 480.0, 0.1, 100.0);
            gl::UniformMatrix4fv(
                projection_location,
                1,
                gl::FALSE as GLboolean,
                projection.as_ptr(),
            );

            // Set up view matrix
            let view_location =
                gl::GetUniformLocation(shader.id, b"view\0".as_ptr() as *const GLchar);
            let view: Matrix4<f32> = Matrix4::look_at_rh(
                Point3::new(0.0, 0.0, 5.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            );
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE as GLboolean, view.as_ptr());

            // Define cube positions and colors
            let cube_positions: [Vector3<f32>; 3] = [
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(2.0, 0.0, -1.0),
                Vector3::new(-2.0, 0.0, -1.0),
            ];

            let cube_colors: [Vector3<f32>; 3] = [
                Vector3::new(1.0, 0.0, 0.0), // Red
                Vector3::new(0.0, 1.0, 0.0), // Green
                Vector3::new(0.0, 0.0, 1.0), // Blue
            ];

            // Render each cube
            for i in 0..cube_count {
                // Model matrix
                let model_location =
                    gl::GetUniformLocation(shader.id, b"model\0".as_ptr() as *const GLchar);
                let model: Matrix4<f32> = Matrix4::from_translation(cube_positions[i])
                    * Matrix4::from_axis_angle(
                        Vector3::new(0.5, 1.0, 0.0).normalize(),
                        Deg(glfw.get_time() as f32 * 50.0),
                    );
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE as GLboolean, model.as_ptr());

                // Color uniform
                let color_location =
                    gl::GetUniformLocation(shader.id, b"color\0".as_ptr() as *const GLchar);
                gl::Uniform4f(
                    color_location,
                    cube_colors[i].x,
                    cube_colors[i].y,
                    cube_colors[i].z,
                    1.0,
                );

                // Draw the cube
                mesh.draw();
            }

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
