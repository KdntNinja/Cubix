extern crate gl;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::cube_render::draw_chunk;
use crate::world::generation::generate_chunk;

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

    pub fn draw(&self, shader: &Shader, time: f32) {
        draw_chunk(&self.chunk_data, &self.mesh, shader, time);
    }
}
