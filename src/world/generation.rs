/// Generates a chunk of blocks.
///
/// # Returns
///
/// A 3D array representing the chunk data.
pub fn generate_chunk() -> [[[u32; 16]; 16]; 16] {
    let mut chunk_data = [[[0; 16]; 16]; 16];

    for x in 0..16 {
        for z in 0..16 {
            // Generate terrain with hills and valleys
            let base_height = 4;

            // Create some height variation using simple noise
            let h1 = (x as f32 * 0.5).sin() * 1.5;
            let h2 = (z as f32 * 0.5).cos() * 1.5;
            let h3 = ((x as f32 * 0.7 + z as f32 * 0.3) * 0.4).sin() * 0.5;

            let height_offset = (h1 + h2 + h3).max(0.0) as usize;
            let height = base_height + height_offset;

            // Generate terrain layers
            for y in 0..16 {
                if y < height - 1 {
                    chunk_data[x][y][z] = 3; // Stone
                } else if y < height {
                    chunk_data[x][y][z] = 1; // Dirt
                } else if y == height {
                    // Add snow on high terrain, grass on lower
                    if height >= 7 {
                        chunk_data[x][y][z] = 4; // Snow
                    } else {
                        chunk_data[x][y][z] = 2; // Grass
                    }
                }

                // Add water pools in low areas
                if y <= 2 && height <= 3 && chunk_data[x][y][z] == 0 {
                    chunk_data[x][y][z] = 5; // Water
                }
            }
        }
    }

    chunk_data
}
