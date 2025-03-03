extern crate gl;

use cgmath::{Matrix, Matrix4, Vector3};
use gl::types::*;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::block::Block;

// New function to draw chunk data
pub fn draw_chunk(chunk_data: &[[[u32; 16]; 16]; 16], mesh: &Mesh, shader: &Shader, _time: f32) {
    unsafe {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block_id = chunk_data[x][y][z];
                    if block_id != 0 {
                        // Model matrix
                        let model_location =
                            gl::GetUniformLocation(shader.id, b"model\0".as_ptr() as *const GLchar);
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
