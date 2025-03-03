pub fn generate_chunk() -> [[[u32; 16]; 16]; 16] {
    let mut chunk_data = [[[0; 16]; 16]; 16];

    for x in 0..16 {
        for z in 0..16 {
            // Example: simple heightmap
            let height = (x + z) % 16;
            for y in 0..height {
                chunk_data[x][y][z] = 1; // Example: block ID 1
            }
        }
    }

    chunk_data
}
