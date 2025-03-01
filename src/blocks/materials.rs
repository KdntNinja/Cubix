use bevy::prelude::*;

#[derive(Resource)]
pub struct BlockMaterials {
    pub normal: Handle<StandardMaterial>,
    pub highlighted: Handle<StandardMaterial>,
}

pub fn setup_block_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create materials for blocks
    let normal_material = materials.add(Color::srgb_u8(124, 144, 255));
    let highlighted_material = materials.add(Color::WHITE);

    commands.insert_resource(BlockMaterials {
        normal: normal_material,
        highlighted: highlighted_material,
    });
}
