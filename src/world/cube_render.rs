extern crate gl;

use cgmath::{Deg, InnerSpace, Matrix, Matrix4, Vector3};
use gl::types::*;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;

pub fn draw_cubes(mesh: &Mesh, shader: &Shader, time: f32) {
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

    unsafe {
        for i in 0..cube_positions.len() {
            // Model matrix
            let model_location =
                gl::GetUniformLocation(shader.id, b"model\0".as_ptr() as *const GLchar);
            let model: Matrix4<f32> = Matrix4::from_translation(cube_positions[i])
                * Matrix4::from_axis_angle(
                    Vector3::new(0.5, 1.0, 0.0).normalize(),
                    Deg(time * 50.0),
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
    }
}
