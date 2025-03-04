extern crate gl;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::cube_render::draw_chunk;
use crate::world::generation::generate_chunk;

/// Represents the game world, including blocks and rendering.
pub struct World {
    pub chunk_data: [[[u32; 16]; 16]; 16],
    pub mesh: Mesh,
}

impl World {
    /// Creates a new `World` instance with the given mesh.
    ///
    /// # Arguments
    ///
    /// * `mesh` - The mesh to use for rendering the world.
    ///
    /// # Returns
    ///
    /// A new `World` instance.
    pub fn new(mesh: Mesh) -> Self {
        World {
            chunk_data: generate_chunk(),
            mesh,
        }
    }

    /// Draws the world using the given shader.
    ///
    /// # Arguments
    ///
    /// * `shader` - The shader to use for rendering.
    /// * `time` - The current time (used for animations).
    pub fn draw(&self, shader: &Shader, time: f32) {
        draw_chunk(&self.chunk_data, &self.mesh, shader, time);
    }
}
