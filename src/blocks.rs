use crate::settings::Settings;
use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

pub fn generate_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
) {
    let block_size = settings.world.block_size;
    let chunk_size = settings.world.chunk_size;

    for z in 0..chunk_size {
        for x in 0..chunk_size {
            commands.spawn((
                Block,
                Mesh3d(meshes.add(Cuboid::new(block_size, block_size, block_size))),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                Transform::from_xyz(x as f32 * block_size, 0.0, z as f32 * block_size),
                Visibility::Visible, // Ensure blocks are visible
            ));
        }
    }
}
