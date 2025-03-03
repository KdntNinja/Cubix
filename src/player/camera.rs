use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub yaw: f32,   // Horizontal rotation (in degrees)
    pub pitch: f32, // Vertical rotation (in degrees)
}

impl Camera {
    pub fn new(position: Point3<f32>) -> Self {
        Camera {
            position,
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            yaw: -90.0, // Start facing negative z
            pitch: 0.0,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn process_mouse_movement(
        &mut self,
        x_offset: f32,
        y_offset: f32,
        constrain_pitch: bool,
        sensitivity: f32,
    ) {
        self.yaw += x_offset * sensitivity;
        self.pitch += y_offset * sensitivity;

        // Constrain pitch to avoid flipping
        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        // Update front vector
        self.update_camera_vectors();
    }

    pub fn update_camera_vectors(&mut self) {
        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
    }
}
