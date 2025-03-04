/// Generates a chunk of blocks.
///
/// # Returns
///
/// A 3D array representing the chunk data.
pub fn generate_chunk() -> [[[u32; 16]; 16]; 16] {
    let mut chunk_data = [[[0; 16]; 16]; 16];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                if y == 0 {
                    chunk_data[x][y][z] = 3; // Stone layer
                } else if y < 4 {
                    chunk_data[x][y][z] = 1; // Dirt layer
                } else if y == 4 {
                    chunk_data[x][y][z] = 2; // Grass layer
                }
            }
        }
    }

    chunk_data
}
