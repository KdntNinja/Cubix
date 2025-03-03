extern crate gl;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::block::Block;
use crate::world::generation::generate_chunk;
use cgmath::Matrix;
use cgmath::{Matrix4, Vector3};
use gl::types::*;

pub struct World {
    pub chunk_data: [[[u32; 16]; 16]; 16],
    pub mesh: Mesh,
}

impl World {
    pub fn new(mesh: Mesh) -> Self {
        World {
            chunk_data: generate_chunk(),
            mesh,
        }
    }

    pub fn draw(&self, shader: &Shader, _time: f32) {
        unsafe {
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block_id = self.chunk_data[x][y][z];
                        if block_id != 0 {
                            // Model matrix
                            let model_location = gl::GetUniformLocation(
                                shader.id,
                                b"model\0".as_ptr() as *const GLchar,
                            );
                            let model: Matrix4<f32> = Matrix4::from_translation(Vector3::new(
                                x as f32, y as f32, z as f32,
                            ));
                            gl::UniformMatrix4fv(
                                model_location,
                                1,
                                gl::FALSE as GLboolean,
                                model.as_ptr(),
                            );

                            // Apply color based on block ID
                            let block = Block::new(block_id);
                            let color = block.get_color();
                            let color_location = gl::GetUniformLocation(
                                shader.id,
                                b"color\0".as_ptr() as *const GLchar,
                            );
                            gl::Uniform4f(color_location, color[0], color[1], color[2], color[3]);

                            // Draw the cube
                            self.mesh.draw();
                        }
                    }
                }
            }
        }
    }
}
