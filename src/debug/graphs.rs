use super::components::*;
use bevy::prelude::*;
use sysinfo::{Pid, System};

// System monitoring resource
#[derive(Resource)]
pub struct SystemInfo {
    pub sys: System,
    pub process_id: Option<Pid>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        // Get our process ID
        let process_id = sysinfo::get_current_pid().ok();

        Self { sys, process_id }
    }
}

pub fn setup_performance_graphs(mut commands: Commands) {
    // Initialize system monitoring
    commands.insert_resource(SystemInfo::default());

    // FPS Graph
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.5),
            custom_size: Some(Vec2::new(200.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(105.0, 180.0, 0.0),
        Visibility::Hidden,
        FpsGraph,
    ));

    // Memory Graph
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.5),
            custom_size: Some(Vec2::new(200.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(105.0, 260.0, 0.0),
        Visibility::Hidden,
        MemoryGraph,
    ));
}

pub fn update_performance_graphs(
    mut commands: Commands,
    time: Res<Time>,
    mut debug_state: ResMut<DebugState>,
    mut sys_info: ResMut<SystemInfo>,
    debug_visible: Query<&Visibility, With<DebugContainer>>,
    mut graph_queries: ParamSet<(
        Query<(Entity, &Visibility), With<FpsGraph>>,
        Query<(Entity, &Visibility), With<MemoryGraph>>,
    )>,
) {
    let is_visible = debug_visible
        .get_single()
        .map_or(false, |v| *v == Visibility::Visible);

    // Only update metrics every 0.25 seconds to avoid excessive CPU usage
    debug_state.last_update += time.delta_secs();
    if debug_state.last_update >= 0.25 {
        debug_state.last_update = 0.0;

        // Update system info
        sys_info.sys.refresh_all();

        // Get real memory usage
        if let Some(pid) = sys_info.process_id {
            if let Some(process) = sys_info.sys.process(pid) {
                // Get memory values
                let process_memory = process.memory() as f64 / 1024.0; // Convert KB to MB
                let total_memory = sys_info.sys.total_memory() as f64 / 1024.0 / 1024.0; // Convert KB to MB

                // Calculate percentage
                let memory_usage = (process_memory / (total_memory * 1024.0) * 100.0) as f32;
                debug_state.memory_usage.push(memory_usage);

                if debug_state.memory_usage.len() > 100 {
                    debug_state.memory_usage.remove(0);
                }

                // Store values in debug_state instead of updating text directly
                debug_state.current_memory_usage = memory_usage;
                debug_state.current_process_memory = process_memory as f32;
                debug_state.current_total_memory = total_memory as f32;
            }
        }
    }

    // Helper function to update visibility
    let update_visibility = |entity: Entity, current: &Visibility, commands: &mut Commands| {
        let should_be_visible = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if *current != should_be_visible {
            commands.entity(entity).insert(should_be_visible);
        }
    };

    // Update FPS graph visibility
    for (entity, visibility) in graph_queries.p0().iter() {
        update_visibility(entity, visibility, &mut commands);
    }

    // Update Memory graph visibility
    for (entity, visibility) in graph_queries.p1().iter() {
        update_visibility(entity, visibility, &mut commands);
    }
}
