use super::components::*;
use bevy::prelude::*;

pub fn update_fps_counter(
    time: Res<Time>,
    mut debug_state: ResMut<DebugState>,
    debug_visible: Query<&Visibility, With<DebugContainer>>,
    mut fps_query: Query<&mut Text, With<FpsText>>,
) {
    // Only update FPS history when debug is visible
    let is_visible = debug_visible
        .get_single()
        .map_or(false, |v| *v == Visibility::Visible);

    if is_visible {
        let fps = 1.0 / time.delta_secs();
        let frame_time = time.delta_secs() * 1000.0;

        // Update FPS history
        debug_state.fps_history.push(fps);
        if debug_state.fps_history.len() > 100 {
            debug_state.fps_history.remove(0);
        }

        // Calculate min/max/avg for display
        let min_fps = debug_state
            .fps_history
            .iter()
            .fold(f32::MAX, |a: f32, &b| a.min(b));
        let max_fps = debug_state
            .fps_history
            .iter()
            .fold(0.0f32, |a, &b| a.max(b));
        let avg_fps =
            debug_state.fps_history.iter().sum::<f32>() / debug_state.fps_history.len() as f32;

        // Update text in Minecraft style
        if let Ok(mut text) = fps_query.get_single_mut() {
            *text = Text::from(format!(
                "{} fps T: {:.1}ms\nMin: {:.0} | Max: {:.0} | Avg: {:.0}",
                fps as u32, frame_time, min_fps, max_fps, avg_fps
            ));
        }
    }
}
