#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub id: u32,
}

impl Block {
    pub fn new(id: u32) -> Self {
        Block { id }
    }

    pub fn get_cube_vertices() -> [f32; 108] {
        [
            // Back face
            -0.5, -0.5, -0.5, // Bottom-left
            0.5, -0.5, -0.5, // Bottom-right
            0.5, 0.5, -0.5, // Top-right
            0.5, 0.5, -0.5, // Top-right
            -0.5, 0.5, -0.5, // Top-left
            -0.5, -0.5, -0.5, // Bottom-left
            // Front face
            -0.5, -0.5, 0.5, // Bottom-left
            0.5, -0.5, 0.5, // Bottom-right
            0.5, 0.5, 0.5, // Top-right
            0.5, 0.5, 0.5, // Top-right
            -0.5, 0.5, 0.5, // Top-left
            -0.5, -0.5, 0.5, // Bottom-left
            // Left face
            -0.5, 0.5, 0.5, // Top-right
            -0.5, 0.5, -0.5, // Top-left
            -0.5, -0.5, -0.5, // Bottom-left
            -0.5, -0.5, -0.5, // Bottom-left
            -0.5, -0.5, 0.5, // Bottom-right
            -0.5, 0.5, 0.5, // Top-right
            // Right face
            0.5, 0.5, 0.5, // Top-left
            0.5, 0.5, -0.5, // Top-right
            0.5, -0.5, -0.5, // Bottom-right
            0.5, -0.5, -0.5, // Bottom-right
            0.5, -0.5, 0.5, // Bottom-left
            0.5, 0.5, 0.5, // Top-left
            // Bottom face
            -0.5, -0.5, -0.5, // Top-right
            0.5, -0.5, -0.5, // Top-left
            0.5, -0.5, 0.5, // Bottom-left
            0.5, -0.5, 0.5, // Bottom-left
            -0.5, -0.5, 0.5, // Bottom-right
            -0.5, -0.5, -0.5, // Top-right
            // Top face
            -0.5, 0.5, -0.5, // Top-left
            0.5, 0.5, -0.5, // Top-right
            0.5, 0.5, 0.5, // Bottom-right
            0.5, 0.5, 0.5, // Bottom-right
            -0.5, 0.5, 0.5, // Bottom-left
            -0.5, 0.5, -0.5, // Top-left
        ]
    }
}
