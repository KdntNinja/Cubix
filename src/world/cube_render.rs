extern crate gl;

use cgmath::{Matrix, Matrix4, Vector3};
use gl::types::*;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::block::Block;

/// Draws a chunk of blocks using the given mesh and shader.
///
/// # Arguments
///
/// * `chunk_data` - A 3D array representing the chunk data.
/// * `mesh` - The mesh to use for rendering.
/// * `shader` - The shader to use for rendering.
/// * `time` - The current time (used for animations).
pub fn draw_chunk(chunk_data: &[[[u32; 16]; 16]; 16], mesh: &Mesh, shader: &Shader, _time: f32) {
    // First pass: Draw solid blocks
    draw_solid_blocks(chunk_data, mesh, shader);

    // Second pass: Draw edges
    draw_block_edges(chunk_data, mesh, shader);
}

/// Draw the solid blocks
///
/// # Arguments
///
/// * `chunk_data` - A 3D array representing the chunk data.
/// * `mesh` - The mesh to use for rendering.
/// * `shader` - The shader to use for rendering.
fn draw_solid_blocks(chunk_data: &[[[u32; 16]; 16]; 16], mesh: &Mesh, shader: &Shader) {
    unsafe {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block_id = chunk_data[x][y][z];
                    if block_id != 0 {
                        // Model matrix - use consistent cube size
                        let model_location =
                            gl::GetUniformLocation(shader.id, b"model\0".as_ptr() as *const GLchar);

                        // Position cubes with exact 1.0 unit size for perfect cubes
                        // Use pure translation without scaling to maintain cube proportions
                        let model: Matrix4<f32> =
                            Matrix4::from_translation(Vector3::new(x as f32, y as f32, z as f32));

                        gl::UniformMatrix4fv(
                            model_location,
                            1,
                            gl::FALSE as GLboolean,
                            model.as_ptr(),
                        );

                        // Apply color based on block ID
                        let block = Block::new(block_id);
                        let color = block.get_color();
                        let color_location =
                            gl::GetUniformLocation(shader.id, b"color\0".as_ptr() as *const GLchar);
                        gl::Uniform4f(color_location, color[0], color[1], color[2], color[3]);

                        // Draw the cube
                        mesh.draw();
                    }
                }
            }
        }
    }
}

/// Draw the edges of blocks
///
/// # Arguments
///
/// * `chunk_data` - A 3D array representing the chunk data.
/// * `mesh` - The mesh to use for rendering.
/// * `shader` - The shader to use for rendering.
fn draw_block_edges(chunk_data: &[[[u32; 16]; 16]; 16], mesh: &Mesh, shader: &Shader) {
    unsafe {
        // Save the current polygon mode
        let mut polygon_mode = [0];
        gl::GetIntegerv(gl::POLYGON_MODE, polygon_mode.as_mut_ptr());

        // Set wireframe mode
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // Set line width for edges - keep this thin for crisp edges
        gl::LineWidth(1.5);

        // Enable polygon offset to prevent z-fighting between edges and faces
        gl::Enable(gl::POLYGON_OFFSET_LINE);
        gl::PolygonOffset(-1.0, -1.0);

        // Set edge color (black)
        let color_location =
            gl::GetUniformLocation(shader.id, b"color\0".as_ptr() as *const GLchar);
        gl::Uniform4f(color_location, 0.0, 0.0, 0.0, 1.0);

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block_id = chunk_data[x][y][z];
                    if block_id != 0 {
                        // Model matrix
                        let model_location =
                            gl::GetUniformLocation(shader.id, b"model\0".as_ptr() as *const GLchar);

                        // Create basic model matrix for position - use same size as solid cubes
                        let model =
                            Matrix4::from_translation(Vector3::new(x as f32, y as f32, z as f32));

                        gl::UniformMatrix4fv(
                            model_location,
                            1,
                            gl::FALSE as GLboolean,
                            model.as_ptr(),
                        );

                        // Draw the wireframe
                        mesh.draw();
                    }
                }
            }
        }

        // Disable polygon offset
        gl::Disable(gl::POLYGON_OFFSET_LINE);

        // Restore original polygon mode
        gl::PolygonMode(gl::FRONT_AND_BACK, polygon_mode[0] as GLenum);
    }
}
