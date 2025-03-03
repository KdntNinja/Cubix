pub fn generate_chunk() -> [[[u32; 16]; 16]; 16] {
    let mut chunk_data = [[[0; 16]; 16]; 16];
    let flat_height = 5; // Constant height for flat terrain

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..flat_height {
                if y == flat_height - 1 {
                    // Top layer is grass
                    chunk_data[x][y][z] = 2;
                } else if y >= flat_height - 3 {
                    // Next 2 layers are dirt
                    chunk_data[x][y][z] = 1;
                } else {
                    // Everything below is stone
                    chunk_data[x][y][z] = 3;
                }
            }
        }
    }

    chunk_data
}
