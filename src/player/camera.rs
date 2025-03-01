use super::components::Player;
use crate::settings::Settings;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub fn mouse_look(
    mut motion_evr: EventReader<MouseMotion>,
    settings: Res<Settings>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mouse_delta = motion_evr.read().fold(Vec2::ZERO, |acc, ev| acc + ev.delta);

    if mouse_delta == Vec2::ZERO {
        return;
    }

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        // Rotate player horizontally
        player_transform.rotate_y(-mouse_delta.x * settings.player.sensitivity);

        // Rotate camera vertically
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Calculate new pitch
            let pitch = (camera_transform.rotation.to_euler(EulerRot::XYZ).0
                - mouse_delta.y * settings.player.sensitivity)
                .clamp(-1.5, 1.5);

            // Apply new pitch
            camera_transform.rotation = Quat::from_euler(EulerRot::XYZ, pitch, 0.0, 0.0);
        }
    }
}
