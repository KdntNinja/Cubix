extern crate gl;

use crate::rendering::mesh::Mesh;
use crate::rendering::shader::Shader;
use crate::world::cube_render::draw_chunk;
use crate::world::generation::generate_chunk;
use cgmath::Point3;

/// Represents the game world, including blocks and rendering.
pub struct World {
    pub chunk_data: [[[u32; 16]; 16]; 16],
    pub mesh: Mesh,
}

impl World {
    /// Creates a new `World` instance with the given mesh.
    ///
    /// # Arguments
    ///
    /// * `mesh` - The mesh to use for rendering the world.
    ///
    /// # Returns
    ///
    /// A new `World` instance.
    pub fn new(mesh: Mesh) -> Self {
        World {
            chunk_data: generate_chunk(),
            mesh,
        }
    }

    /// Resolves collision and returns a safe position
    pub fn resolve_collision(
        &self,
        current_pos: Point3<f32>,
        target_pos: Point3<f32>,
        radius: f32,
        height: f32,
    ) -> Point3<f32> {
        // Convert camera position (eyes) to feet position for collision
        let feet_current = Point3::new(current_pos.x, current_pos.y - height, current_pos.z);
        let feet_target = Point3::new(target_pos.x, target_pos.y - height, target_pos.z);

        // If no collision at target position, return it
        if !self.check_collision(&feet_target, radius, height) {
            return target_pos;
        }

        // Try horizontal movement separately to enable sliding
        let mut new_feet_pos = feet_current;

        // Try x-axis movement
        let x_pos = Point3::new(feet_target.x, feet_current.y, feet_current.z);
        if !self.check_collision(&x_pos, radius, height) {
            new_feet_pos.x = feet_target.x;
        }

        // Try z-axis movement
        let z_pos = Point3::new(new_feet_pos.x, feet_current.y, feet_target.z);
        if !self.check_collision(&z_pos, radius, height) {
            new_feet_pos.z = feet_target.z;
        }

        // Try vertical movement (for jumping/falling)
        let y_pos = Point3::new(new_feet_pos.x, feet_target.y, new_feet_pos.z);
        if !self.check_collision(&y_pos, radius, height) {
            new_feet_pos.y = feet_target.y;
        }

        // Convert back to camera/eye position
        Point3::new(new_feet_pos.x, new_feet_pos.y + height, new_feet_pos.z)
    }

    pub fn check_collision(&self, feet_position: &Point3<f32>, radius: f32, height: f32) -> bool {
        // Convert feet position to block coordinates
        let block_x = feet_position.x.floor() as usize;
        let block_y = feet_position.y.floor() as usize;
        let block_z = feet_position.z.floor() as usize;

        // Check blocks from feet position upward
        let radius_check = radius.ceil() as usize + 1;

        for x in block_x.saturating_sub(radius_check)..=(block_x + radius_check).min(15) {
            for y in block_y..=(block_y + height as usize).min(15) {
                for z in block_z.saturating_sub(radius_check)..=(block_z + radius_check).min(15) {
                    // Skip air blocks
                    if self.chunk_data[x][y][z] == 0 {
                        continue;
                    }

                    // Check collision with this block
                    if self.check_block_collision(feet_position, x, y, z, radius, height) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn check_block_collision(
        &self,
        feet_position: &Point3<f32>,
        block_x: usize,
        block_y: usize,
        block_z: usize,
        radius: f32,
        height: f32,
    ) -> bool {
        // Block min/max coordinates
        let block_min_x = block_x as f32;
        let block_max_x = block_x as f32 + 1.0;
        let block_min_y = block_y as f32;
        let block_max_y = block_y as f32 + 1.0;
        let block_min_z = block_z as f32;
        let block_max_z = block_z as f32 + 1.0;

        // Player capsule min/max (from feet position)
        let player_min_y = feet_position.y;
        let player_max_y = feet_position.y + height;

        // Check vertical overlap first (faster rejection)
        if player_max_y <= block_min_y || player_min_y >= block_max_y {
            return false;
        }

        // For horizontal collision, treat player as a cylinder
        let closest_x = feet_position.x.max(block_min_x).min(block_max_x);
        let closest_z = feet_position.z.max(block_min_z).min(block_max_z);

        // Distance from the closest point to player center axis
        let dx = closest_x - feet_position.x;
        let dz = closest_z - feet_position.z;
        let distance_squared = dx * dx + dz * dz;

        // Collision if distance is less than radius
        distance_squared < (radius * radius)
    }

    /// Draws the world using the given shader.
    ///
    /// # Arguments
    ///
    /// * `shader` - The shader to use for rendering.
    /// * `time` - The current time (used for animations).
    pub fn draw(&self, shader: &Shader, time: f32) {
        draw_chunk(&self.chunk_data, &self.mesh, shader, time);
    }
}
