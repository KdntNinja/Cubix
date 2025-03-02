use super::components::Player;
use crate::settings::Settings;
use bevy::prelude::*;

pub fn setup_cursor(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
) {
    // Position player higher above the ground to ensure it doesn't immediately collide
    let start_height = settings.world.block_size * 5.0;

    // Center player over the block grid
    let center_x = (settings.world.chunk_size as f32 * settings.world.block_size) / 2.0;
    let center_z = center_x; // Assuming square grid

    commands
        .spawn((
            Transform::from_xyz(center_x, start_height, center_z), // Center player over blocks
            Visibility::Visible,
            Player {
                velocity: Vec3::ZERO,
                grounded: false,
            },
        ))
        .with_children(|parent| {
            // Player model (cube)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.8, 1.8, 0.8))),
                MeshMaterial3d(materials.add(Color::srgb_u8(255, 100, 100))),
                Transform::default(),
            ));

            // Camera as child of player at eye level
            parent.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.6, 0.0)));
        });
}
