use super::components::Block;
use super::materials::BlockMaterials;
use crate::settings::Settings;
use bevy::prelude::*;

pub fn generate_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    block_materials: Res<BlockMaterials>,
    settings: Res<Settings>,
) {
    let block_size = settings.world.block_size;
    let chunk_size = settings.world.chunk_size;
    let grid_offset = 0.02; // Small gap between blocks for grid effect

    for z in 0..chunk_size {
        for x in 0..chunk_size {
            // Create slightly smaller blocks to create visual grid lines
            let visual_size = block_size - grid_offset;

            commands.spawn((
                Block,
                Mesh3d(meshes.add(Cuboid::new(visual_size, visual_size, visual_size))),
                MeshMaterial3d(block_materials.normal.clone()),
                Transform::from_xyz(x as f32 * block_size, 0.0, z as f32 * block_size),
                Visibility::Visible,
            ));
        }
    }
}
