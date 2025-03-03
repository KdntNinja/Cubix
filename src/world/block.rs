#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub id: u32,
}

impl Block {
    pub fn new(id: u32) -> Self {
        Block { id }
    }

    pub fn get_color(&self) -> [f32; 4] {
        match self.id {
            0 => [0.0, 0.0, 0.0, 0.0], // Air/empty (transparent)
            1 => [0.6, 0.3, 0.0, 1.0], // Dirt/soil (brown)
            2 => [0.0, 0.7, 0.0, 1.0], // Grass (green)
            3 => [0.5, 0.5, 0.5, 1.0], // Stone (gray)
            4 => [0.9, 0.9, 0.9, 1.0], // Snow (white)
            5 => [0.0, 0.0, 0.8, 1.0], // Water (blue)
            _ => [1.0, 0.0, 1.0, 1.0], // Unknown (magenta)
        }
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
