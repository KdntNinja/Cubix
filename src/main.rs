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
        .add_systems(Startup, setup)
        .add_systems(Update, exit_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
) {
    // blocks
    generate_chunk(&mut commands, meshes, materials, settings);

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
