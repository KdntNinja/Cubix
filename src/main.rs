mod blocks;
mod helper;
mod player;
mod settings;

use bevy::prelude::*;
use blocks::*;
use helper::*;
use player::*;
use settings::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SettingsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BlocksPlugin)
        .add_systems(Startup, setup.after(setup_block_materials))
        .add_systems(Update, exit_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    block_materials: Res<BlockMaterials>,
    settings: Res<Settings>,
) {
    // blocks
    generate_chunk(&mut commands, meshes, block_materials, settings);

    // Better lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 8.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
