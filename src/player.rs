use crate::blocks::Block;
use crate::settings::Settings;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, setup_cursor))
            // Apply gravity first, then handle collisions, then process movement
            .add_systems(
                Update,
                (
                    apply_gravity,
                    handle_collisions,
                    player_movement,
                    mouse_look,
                )
                    .chain(),
            ); // Chain ensures they run in this order
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub grounded: bool,
}

fn setup_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
) {
    // Position player higher above the ground to ensure it doesn't immediately collide
    let start_height = settings.world.block_size * 5.0; // Increase from 2.0 to 5.0

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

fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Forward/backward movement (Z axis)
        if keyboard.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }

        // Left/right movement (X axis)
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Jump only when grounded
        if keyboard.pressed(KeyCode::Space) && player.grounded {
            player.velocity.y = settings.player.jump_force;
            player.grounded = false;
        }

        // Apply horizontal movement
        if direction.x != 0.0 || direction.z != 0.0 {
            // Transform the movement direction based on player rotation
            let forward = transform.forward();
            let right = transform.right();

            let horizontal_movement = (forward * -direction.z) + (right * direction.x);

            if horizontal_movement != Vec3::ZERO {
                let normalized = horizontal_movement.normalize();
                player.velocity.x = normalized.x * settings.player.speed;
                player.velocity.z = normalized.z * settings.player.speed;
            }
        } else {
            // Slow down horizontal movement
            player.velocity.x *= 0.9;
            player.velocity.z *= 0.9;
        }

        // Apply velocity
        transform.translation += player.velocity * time.delta_secs();
    }
}

fn apply_gravity(time: Res<Time>, settings: Res<Settings>, mut player_query: Query<&mut Player>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if !player.grounded {
            player.velocity.y -= settings.player.gravity * time.delta_secs();
        }
    }
}

fn handle_collisions(
    settings: Res<Settings>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    block_query: Query<&Transform, (With<Block>, Without<Player>)>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let player_pos = player_transform.translation;
        let player_half_height = 0.9; // Half of player height
        let player_half_width = 0.4; // Half of player width
        let block_size = settings.world.block_size;

        // Reset grounded status but check for ground below first
        let was_grounded = player.grounded;
        player.grounded = false;

        // Ground detection buffer - add a small distance to check below player
        let ground_check_distance = 0.05;

        for block_transform in block_query.iter() {
            let block_pos = block_transform.translation;

            // Check if player is within collision range of this block
            let dx = (player_pos.x - block_pos.x).abs();
            let dy = (player_pos.y - block_pos.y).abs();
            let dz = (player_pos.z - block_pos.z).abs();

            // Ground specific check with a bit of buffer
            if dx < (player_half_width + block_size / 2.0 - 0.05)
                && dz < (player_half_width + block_size / 2.0 - 0.05)
                && player_pos.y > block_pos.y
                && (player_pos.y - player_half_height - block_pos.y - block_size / 2.0).abs()
                    <= ground_check_distance
            {
                player.grounded = true;

                // Snap to ground to prevent floating point jitter
                if player.velocity.y <= 0.0 {
                    player_transform.translation.y =
                        block_pos.y + block_size / 2.0 + player_half_height + 0.001;
                    player.velocity.y = 0.0;
                    continue; // Skip regular collision for ground snapping
                }
            }

            let collision_x = dx < (player_half_width + block_size / 2.0);
            let collision_y = dy < (player_half_height + block_size / 2.0);
            let collision_z = dz < (player_half_width + block_size / 2.0);

            // Collision detected
            if collision_x && collision_y && collision_z {
                // Find collision direction (smallest penetration)
                let pen_x = player_half_width + block_size / 2.0 - dx;
                let pen_y = player_half_height + block_size / 2.0 - dy;
                let pen_z = player_half_width + block_size / 2.0 - dz;

                // Resolve the collision based on the smallest penetration
                if pen_x <= pen_y && pen_x <= pen_z {
                    // X-axis collision
                    if player_pos.x > block_pos.x {
                        player_transform.translation.x += pen_x;
                    } else {
                        player_transform.translation.x -= pen_x;
                    }
                    player.velocity.x = 0.0;
                } else if pen_y <= pen_x && pen_y <= pen_z {
                    // Y-axis collision
                    if player_pos.y > block_pos.y {
                        player_transform.translation.y += pen_y;
                        player.grounded = true;
                        player.velocity.y = 0.0;
                    } else {
                        player_transform.translation.y -= pen_y;
                        player.velocity.y = 0.0;
                    }
                } else {
                    // Z-axis collision
                    if player_pos.z > block_pos.z {
                        player_transform.translation.z += pen_z;
                    } else {
                        player_transform.translation.z -= pen_z;
                    }
                    player.velocity.z = 0.0;
                }
            }
        }

        // If we just left the ground, add a small leeway period
        if was_grounded && !player.grounded && player.velocity.y <= 0.0 {
            player.grounded = true;
            player.velocity.y = 0.0;
        }
    }
}

fn mouse_look(
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
        // Rotate player horizontally (Y axis)
        player_transform.rotate_y(-mouse_delta.x * settings.player.sensitivity * 0.01);

        // Rotate camera vertically (X axis)
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let current_x_rotation = camera_transform.rotation.x;
            let new_x_rotation = (current_x_rotation
                - mouse_delta.y * settings.player.sensitivity * 0.01)
                .clamp(-1.5, 1.5);

            camera_transform.rotation.x = new_x_rotation;
        }
    }
}
